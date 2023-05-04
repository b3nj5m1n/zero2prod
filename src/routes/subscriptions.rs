use axum::{extract::State, http::StatusCode, Form};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Subscriber {
    name: String,
    email: String,
}

pub async fn subscribe(
    State(connection): State<PgPool>,
    Form(subscriber): Form<Subscriber>,
) -> StatusCode {
    info!("Subbing {subscriber:?}");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscriber.email,
        subscriber.name,
        Utc::now()
    )
    .execute(&connection)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
