use crate::routes::{root, subscribe};
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::{net::TcpListener, sync::Arc};

pub fn app(connection: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/subscriptions", post(subscribe))
        .with_state(connection)
    // .layer(TraceLayer::new_for_http())
}

pub async fn run(listener: TcpListener, pool: PgPool) -> std::io::Result<()> {
    let pool = Arc::new(pool);
    axum_server::from_tcp(listener)
        .serve(app(pool).into_make_service())
        .await
}
