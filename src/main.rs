use production::{configuration::get_configuration, startup::run};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).unwrap();

    let pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to get DB connection");
    println!("running on port: {}", listener.local_addr().unwrap());
    run(listener, pool).await.unwrap();
}
