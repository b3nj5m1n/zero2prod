use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let addr = listener.local_addr().unwrap();
    let server = zero2prod::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://{}", addr)
}

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{addr}/health_check"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let addr = spawn_app();
    let config = get_configuration().expect("Failed to read configuration");
    let conn_string = config.database.connection_string();

    let mut connection = PgConnection::connect(&conn_string)
        .await
        .expect("Failed to connect to postgres");

    let client = reqwest::Client::new();

    let body = "name=benjamin&email=b3nj4m1n%40gmx.net";
    let response = client
        .post(format!("{addr}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "b3nj4m1n@gmx.net");
    assert_eq!(saved.name, "benjamin");
}

#[tokio::test]
async fn subscribe_returns_422_when_data_is_missing() {
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{addr}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did not fail with 422 Unprocessable Entity when the payload was {error_message}."
        )
    }
}