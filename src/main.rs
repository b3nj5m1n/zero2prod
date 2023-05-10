use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;
use zero2prod::{
    configuration::get_configuration,
    email_client::EmailClient,
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

    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))?;
    let config = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());

    let sender_email = config.email_client.sender().expect("Invalid sender email");
    let email_client = EmailClient::new(
        config.email_client.base_url.clone(),
        sender_email,
        config.email_client.api_key.clone(),
        config.email_client.timeout(),
    );

    zero2prod::startup::run(listener, connection_pool, email_client)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
}
