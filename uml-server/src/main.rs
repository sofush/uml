use actix_files::Files;
use actix_files::NamedFile;
use actix_web::rt;
use actix_web::web::Data;
use actix_web::web::Payload;
use actix_web::{
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder, middleware,
    web,
};
use client_handler::ClientHandler;
use env_logger::Env;
use state::State;
use tokio::sync::Mutex;

mod client_handler;
mod state;

async fn index() -> impl Responder {
    NamedFile::open_async("./uml-server/static/index.html")
        .await
        .unwrap()
}

pub async fn websocket(
    req: HttpRequest,
    stream: Payload,
    state: Data<Mutex<State>>,
) -> Result<HttpResponse, Error> {
    let (res, session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    let task_handle = rt::spawn(async move {
        let mut handler = ClientHandler::new(session);

        while let Some(msg) = stream.recv().await {
            let res = match msg {
                Ok(m) => handler.handle(m).await,
                Err(e) => {
                    log::error!("Websocket error: {e}");
                    break;
                }
            };

            if let Err(actix_ws::Closed) = res {
                break;
            }
        }
    });

    state.get_ref().lock().await.add_connection(task_handle);
    Ok(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::new());

    let data = Data::new(Mutex::new(State::default()));
    let data_clone = data.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(web::resource("/").to(index))
            .service(
                Files::new("/static", "./uml-server/static").prefer_utf8(true),
            )
            .service(Files::new("/wasm", "./uml-wasm/wasm").prefer_utf8(true))
            .service(web::resource("/websocket").to(websocket))
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    data_clone.get_ref().lock().await.close_connections();
    Ok(())
}
