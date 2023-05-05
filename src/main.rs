use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::{
    configuration::get_configuration,
    telemetry::{get_log_file, get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        "zero2prod".into(),
        "info".into(),
        get_log_file("normal".into()).expect("Failed to get log file"),
    );
    init_subscriber(subscriber);
    let config = get_configuration().expect("Failed to read config");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.application_port))?;
    let config = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&config.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to postgres");

    zero2prod::startup::run(listener, connection_pool)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
}
