use axum::{http::StatusCode, Form};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct Subscriber {
    name: String,
    email: String,
}

pub async fn subscribe(Form(subscriber): Form<Subscriber>) -> StatusCode {
    info!("Subbing {subscriber:?}");
    StatusCode::OK
}
