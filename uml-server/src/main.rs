use actix_files::Files;
use actix_files::NamedFile;
use actix_web::rt;
use actix_web::{
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder, middleware,
    web,
};
use actix_ws::AggregatedMessage;
use env_logger::Env;
use futures_util::FutureExt;
use futures_util::StreamExt;
use futures_util::pin_mut;
use state::State;
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::channel;

mod state;

async fn index() -> impl Responder {
    NamedFile::open_async("./uml-server/static/index.html")
        .await
        .unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::new());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State {}))
            .service(web::resource("/").to(index))
            .service(
                Files::new("/static", "./uml-server/static").prefer_utf8(true),
            )
            .service(Files::new("/wasm", "./uml-wasm/wasm").prefer_utf8(true))
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
