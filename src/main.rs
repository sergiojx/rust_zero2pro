// use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

// async fn health_check() -> impl Responder {
//     HttpResponse::Ok()
// }
use std::net::TcpListener;
use zero2prod::startup::fun;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind random port");
    
    // Bubble up the io::Error if we failed to bind to address
    // Otherwise call .await on our See
    fun(listener)?.await
}
