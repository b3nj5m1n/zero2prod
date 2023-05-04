use std::net::SocketAddr;

use anyhow::Result;
use axum::{http::StatusCode, routing::get, Router};

pub fn run(
) -> Result<axum::Server<hyper::server::conn::AddrIncoming, axum::routing::IntoMakeService<Router>>>
{
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new().route("/health_check", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    Ok(axum::Server::bind(&addr).serve(app.into_make_service()))
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
