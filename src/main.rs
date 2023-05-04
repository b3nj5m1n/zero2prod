use std::net::TcpListener;

use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read config");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.application_port))?;
    zero2prod::startup::run(listener)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
}
