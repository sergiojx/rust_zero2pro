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
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // panic if we can not read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Hard coded 8080 is now optained from our settigs
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    
    // Bubble up the io::Error if we failed to bind to address
    // Otherwise call .await on our See
    fun(listener)?.await
}
