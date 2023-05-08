use anyhow::Result;
use reqwest::Client;
use secrecy::{ExposeSecret, Secret};
use serde::Serialize;

use crate::domain::SubscriberEmail;

pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    api_key: Secret<String>,
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail, api_key: Secret<String>) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
            api_key,
        }
    }
    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<()> {
        let url = format!("{}/email/send", self.base_url);
        let send_email_form = SendEmailForm {
            apikey: self.api_key.expose_secret().to_owned(), // Should probably implement SerializableSecret on a custom type instead
            subject: subject.to_owned(),
            from: self.sender.as_ref().to_owned(),
            to: recipient.as_ref().to_owned(),
            body_html: html_content.to_owned(),
            body_text: text_content.to_owned(),
            is_transactional: true,
        };
        let builder = self
            .http_client
            .post(&url)
            .form(&send_email_form)
            .send()
            .await?;
        Ok(())
    }
}

#[derive(Serialize)]
struct SendEmailForm {
    apikey: String,
    subject: String,
    from: String,
    to: String,
    #[serde(rename = "bodyHtml")]
    body_html: String,
    #[serde(rename = "bodyText")]
    body_text: String,
    #[serde(rename = "isTransactional")]
    is_transactional: bool,
}

#[cfg(test)]
mod tests {
    use fake::{
        faker::{
            internet::en::SafeEmail,
            lorem::en::{Paragraph, Sentence},
        },
        Fake,
    };
    use secrecy::Secret;
    use wiremock::{
        matchers::{header, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    use crate::{domain::SubscriberEmail, email_client::EmailClient};

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client =
            EmailClient::new(mock_server.uri(), sender, Secret::new("abcdef".into()));

        Mock::given(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(path("/email/send"))
            .and(method("POST"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content)
            .await;

        // Assertions automatically done by mock
    }
}
