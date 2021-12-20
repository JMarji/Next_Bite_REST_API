//! src/routes/sign_up.rs

use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
pub async fn sign_up(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
    INSERT INTO users (id, email, name, registered_at)
    VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            print!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
    // We use 'get_ref' to get an immutable reference to the 'PgConnection'
    // wrapped by 'web::Data'
}
