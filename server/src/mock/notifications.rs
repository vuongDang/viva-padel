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
    async fn send_notification(
        &self,
        tokens: &[String],
        title: &str,
        body: &str,
        data: Option<serde_json::Value>,
    ) -> Result<(), String> {
        dbg!(&data);
        let notification = notification_request_payload(tokens, title, body, data);
        tracing::info!("MOCK notifications: {:?}", &notification);
        self.notifications.lock().await.push(notification);
        Ok(())
    }
}
