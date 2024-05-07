use axum::{extract::State, Form};
use chrono::Utc;
use reqwest::StatusCode;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

#[axum::debug_handler]
pub async fn subscribe(connection: State<Arc<PgPool>>, Form(form): Form<FormData>) -> StatusCode {
    match sqlx::query!(
        r#"
INSERT INTO subscriptions (id, email, name, subscribed_at)
VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.as_ref())
    .await
    {
        Ok(_) => {
            println!("insert succeed!");
            return StatusCode::OK;
        }
        Err(e) => {
            println!("Failed to execute query: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }
}

// async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
//     let user = User {
//         id: 1337,
//         username: payload.username,
//     };
//
//     // this will be converted into a JSON response
//     // with a status code of `201 Created`
//     (StatusCode::CREATED, Json(user))
// }
//
// #[derive(Deserialize)]
// struct CreateUser {
//     username: String,
// }
//
// #[derive(Serialize)]
// struct User {
//     id: u64,
//     username: String,
// }
