use std::any::Any;

use async_trait::async_trait;
use serde_json::Value;

const EXPO_PUSH_API_URL: &str = "https://exp.host/--/api/v2/push/send";
#[async_trait]
pub trait NotificationsService: Send + Sync + Any {
    async fn send_notification(
        &self,
        tokens: &[String],
        title: &str,
        body: &str,
        data: Option<serde_json::Value>,
    ) -> Result<(), String>;
}

pub struct ExpoNotificationsService;

#[async_trait]
impl NotificationsService for ExpoNotificationsService {
    #[tracing::instrument(name = "Sending notification", skip(self, tokens, title, data))]
    async fn send_notification(
        &self,
        tokens: &[String],
        title: &str,
        body: &str,
        data: Option<serde_json::Value>,
    ) -> Result<(), String> {
        if tokens.is_empty() {
            return Ok(());
        }

        let client = reqwest::Client::new();

        let payload_map = notification_request_payload(tokens, title, body, data);

        match client
            .post(EXPO_PUSH_API_URL)
            .json(&payload_map)
            .send()
            .await
        {
            Ok(resp) => {
                if !resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    Err(format!("Expo API error: {}", text))
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

pub fn notification_request_payload(
    tokens: &[String],
    title: &str,
    body: &str,
    data: Option<serde_json::Value>,
) -> Value {
    let mut payload_map = serde_json::json!({
        "to": tokens,
        "title": title,
        "body": body,
        "sound": "default",
    });

    if let Some(d) = data {
        payload_map
            .as_object_mut()
            .unwrap()
            .insert("data".to_string(), d);
    }
    payload_map
}
