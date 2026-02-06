use tokio::sync::Mutex;

use async_trait::async_trait;
use serde_json::Value;

use crate::services::{NotificationsService, notifications::notification_request_payload};

#[derive(Default)]
pub struct MockNotificationsService {
    pub notifications: Mutex<Vec<Value>>,
}

#[async_trait]
impl NotificationsService for MockNotificationsService {
    #[tracing::instrument(name = "Sending notification", skip(self, tokens, title, data))]
    async fn send_notification(
        &self,
        tokens: &[String],
        title: &str,
        body: &str,
        data: Option<serde_json::Value>,
    ) -> Result<(), String> {
        let notif = notification_request_payload(tokens, title, body, data);
        self.notifications.lock().await.push(notif);
        Ok(())
    }
}
