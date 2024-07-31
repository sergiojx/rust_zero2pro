use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::PgPool;


use crate::routes::*;

pub fn fun(listener: TcpListener, db_connection_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let db_connection_pool_data_wrapper = web::Data::new(db_connection_pool);
    let server = HttpServer::new(move|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Get a pointer copy and attach it to the application state
            .app_data(db_connection_pool_data_wrapper.clone())
    })
    .listen(listener)?
    .run();
    // No .await here!
    Ok(server)
}