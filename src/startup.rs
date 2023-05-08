use std::{net::TcpListener, sync::Arc};

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

use crate::{email_client::EmailClient, routes::*};

pub struct AppState {
    pub connection: PgPool,
    pub email_client: EmailClient,
}

pub fn run(
    listener: TcpListener,
    connection: PgPool,
    email_client: EmailClient,
) -> Result<axum::Server<hyper::server::conn::AddrIncoming, axum::routing::IntoMakeService<Router>>>
{
    let state = AppState {
        connection,
        email_client,
    };
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(Arc::new(state))
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
