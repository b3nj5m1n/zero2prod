use std::{net::TcpListener, sync::Arc};

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::{
    request_id::MakeRequestUuid,
    trace::{DefaultMakeSpan, TraceLayer},
    ServiceBuilderExt,
};

use crate::{
    configuration::{DatabaseSettings, Settings},
    email_client::EmailClient,
    routes::*,
};

type Server =
    axum::Server<hyper::server::conn::AddrIncoming, axum::routing::IntoMakeService<Router>>;

pub struct App {
    port: u16,
    server: Server,
}

impl App {
    pub async fn build(config: &Settings) -> Result<Self> {
        let connection_pool = get_connection_pool(&config.database);
        let sender_email = config.email_client.sender().expect("Invalid sender email");
        let email_client = EmailClient::new(
            config.email_client.base_url.clone(),
            sender_email,
            config.email_client.api_key.clone(),
            config.email_client.timeout(),
        );

        let listener = TcpListener::bind(format!(
            "{}:{}",
            config.application.host, config.application.port
        ))?;
        let port = listener.local_addr()?.port();
        let server = run(listener, connection_pool, email_client)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<()> {
        self.server.await.map_err(|e| anyhow::Error::from(e))
    }
}

pub struct AppState {
    pub connection: PgPool,
    pub email_client: EmailClient,
}

pub fn run(listener: TcpListener, connection: PgPool, email_client: EmailClient) -> Result<Server> {
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

pub fn get_connection_pool(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(config.with_db())
}
