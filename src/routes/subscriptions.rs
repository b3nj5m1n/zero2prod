use anyhow::Result;
use axum::{extract::State, http::StatusCode, Form};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{error, instrument};
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

#[derive(Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = anyhow::Error;

    fn try_from(value: FormData) -> std::result::Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(NewSubscriber { email, name })
    }
}

#[instrument(name = "Adding new subscriber", skip(connection))]
pub async fn subscribe(State(connection): State<PgPool>, Form(form): Form<FormData>) -> StatusCode {
    let new_subscriber = if let Ok(new_sub) = form.try_into() {
        new_sub
    } else {
        return StatusCode::BAD_REQUEST;
    };
    match insert_subscriber(&connection, &new_subscriber).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[instrument(name = "Saving new subscriber in database", skip(connection))]
pub async fn insert_subscriber(connection: &PgPool, subscriber: &NewSubscriber) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscriber.email.as_ref(),
        subscriber.name.as_ref(),
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
