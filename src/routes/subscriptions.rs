use actix_web::{web, HttpResponse};
use sqlx::PgConnection;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// All arguments in the signature of a route handler must implement the FromRequest trait
// actix-web will invoke from_request for each argument and run the actual handler function.
pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
