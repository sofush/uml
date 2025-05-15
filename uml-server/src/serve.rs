use actix_web::{HttpResponse, Responder, web};
use include_dir::{Dir, include_dir};

static STATIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/static");

async fn serve_file(path: String) -> impl Responder {
    let Some(file) = STATIC_DIR.get_file(&path) else {
        log::debug!("Requested file was not found: {path:?}");
        return HttpResponse::NotFound().body("404 not Found");
    };

    let body = file.contents();
    let content_type = mime_guess::from_path(&path).first_or_octet_stream();

    HttpResponse::Ok()
        .content_type(content_type.as_ref())
        .body(body.to_owned())
}

pub async fn serve_static(path: web::Path<String>) -> impl Responder {
    serve_file(path.to_string()).await
}

pub async fn index() -> impl Responder {
    serve_file("index.html".into()).await
}
