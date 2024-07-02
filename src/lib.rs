use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form::<FormData>) -> HttpResponse {

    // HttpResponse::Ok().finish()
    HttpResponse::Ok().body(format!("name: {} email: {}", _form.name, _form.email))

}

pub fn fun(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    // No .await here!
    Ok(server)
}