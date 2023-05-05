use anyhow::Result;
use axum::{extract::State, http::StatusCode, Form};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{error, instrument};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Subscriber {
    name: String,
    email: String,
}

#[instrument(name = "Adding new subscriber", skip(connection))]
pub async fn subscribe(
    State(connection): State<PgPool>,
    Form(subscriber): Form<Subscriber>,
) -> StatusCode {
    match insert_subscriber(&connection, &subscriber).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[instrument(name = "Saving new subscriber in database", skip(connection))]
pub async fn insert_subscriber(connection: &PgPool, subscriber: &Subscriber) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscriber.email,
        subscriber.name,
        Utc::now()
    )
    .execute(connection)
    .await
    .map_err(|e| {
        error!("Failed to execute query: {e:?}");
        e
    })?;
    Ok(())
}
