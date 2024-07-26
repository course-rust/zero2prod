use serde::Deserialize;
use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self, get, post},
    App, HttpResponse, HttpServer,
};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct FormData {
    name: String,
    email: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", get().to(health_check))
            .route("/subscriptions", post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
