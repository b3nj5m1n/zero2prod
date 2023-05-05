use std::net::TcpListener;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::{
    request_id::MakeRequestUuid,
    trace::{DefaultMakeSpan, TraceLayer},
    ServiceBuilderExt,
};

use crate::routes::*;

pub fn run(
    listener: TcpListener,
    connection: PgPool,
) -> Result<axum::Server<hyper::server::conn::AddrIncoming, axum::routing::IntoMakeService<Router>>>
{
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(connection)
        .layer(
            tower::ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true)),
                )
                .propagate_x_request_id(),
        );

    tracing::info!("listening on {}", listener.local_addr()?);
    Ok(axum::Server::from_tcp(listener)?.serve(app.into_make_service()))
}
