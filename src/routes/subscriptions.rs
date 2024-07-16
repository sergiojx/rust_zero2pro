use actix_web::{web, HttpResponse};


use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String
}

pub async fn subscribe(_form: web::Form::<FormData>) -> HttpResponse {

    // HttpResponse::Ok().finish()
    HttpResponse::Ok().body(format!("name: {} email: {}", _form.name, _form.email))

}