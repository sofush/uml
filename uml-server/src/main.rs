use actix_web::rt;
use actix_web::web::Data;
use actix_web::web::Payload;
use actix_web::{
    App, Error, HttpRequest, HttpResponse, HttpServer, middleware, web,
};
use actix_web_static_files::ResourceFiles;
use env_logger::Env;
use futures_util::StreamExt;
use state::State;
use tokio::sync::Mutex;

mod client_handler;
mod id;
mod state;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

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

    HttpServer::new(move || {
        let static_files = gen_wasm_files();

        App::new()
            .app_data(data.clone())
            .service(ResourceFiles::new("/static", static_files))
            .service(web::resource("/websocket").to(websocket))
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    data_clone.get_ref().lock().await.close_connections().await;
    Ok(())
}
