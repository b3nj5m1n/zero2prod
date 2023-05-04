use std::net::TcpListener;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::routes::*;

pub fn run(
    listener: TcpListener,
    connection: PgPool,
) -> Result<axum::Server<hyper::server::conn::AddrIncoming, axum::routing::IntoMakeService<Router>>>
{
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(connection);

    tracing::info!("listening on {}", listener.local_addr()?);
    Ok(axum::Server::from_tcp(listener)?.serve(app.into_make_service()))
}
