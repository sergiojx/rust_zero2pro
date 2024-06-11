// use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

// async fn health_check() -> impl Responder {
//     HttpResponse::Ok()
// }

use zero2prod::fun;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Bubble up the io::Error if we failed to bind to address
    // Otherwise call .await on our Server
    fun()?.await
}
