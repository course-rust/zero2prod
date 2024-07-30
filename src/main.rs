use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{get_configuration, startup::run};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to load configuration");
    let connection = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).unwrap();
    run(listener, connection)?.await
}
