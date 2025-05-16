use actix_web::rt;
use actix_web::web::Data;
use actix_web::web::Payload;
use actix_web::{
    App, Error, HttpRequest, HttpResponse, HttpServer, middleware, web,
};
use env_logger::Env;
use futures_util::StreamExt;
use state::State;
use tokio::sync::Mutex;

mod client_handler;
mod id;
mod serve;
mod state;

pub async fn websocket(
    req: HttpRequest,
    stream: Payload,
    state: Data<Mutex<State>>,
) -> Result<HttpResponse, Error> {
    let (res, session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20))
        .fuse();

    let (tx, rx) = tokio::sync::mpsc::channel(1000);

    rt::spawn(async move {
        while let Some(Ok(msg)) = stream.next().await {
            if tx.send(msg).await.is_err() {
                break;
            };
        }
    });

    state
        .get_ref()
        .lock()
        .await
        .add_connection(session, rx)
        .await;

    Ok(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::new());

    let data = Data::new(Mutex::new(State::default()));
    let data_clone = data.clone();
    let ip = match cfg!(debug_assertions) {
        true => "127.0.0.1",
        false => "0.0.0.0",
    };

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(web::resource("/websocket").to(websocket))
            .service(web::resource("/").to(serve::index))
            .service(
                web::resource("/static/{filename:.*}").to(serve::serve_static),
            )
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
    })
    .bind((ip, 8080))?
    .run()
    .await?;

    data_clone.lock().await.stop().await;
    Ok(())
}
