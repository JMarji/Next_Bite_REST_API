//! src/routes/sign_up.rs

use actix_web::{web, HttpResponse};
use serde;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
pub async fn sign_up(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
