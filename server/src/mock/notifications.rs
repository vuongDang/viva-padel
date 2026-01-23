use async_trait::async_trait;

use crate::services::NotificationsService;

pub struct TestNotificationsService;

#[async_trait]
impl NotificationsService for TestNotificationsService {
    async fn send_notification(
        &self,
        tokens: &[String],
        title: &str,
        body: &str,
        _data: Option<serde_json::Value>,
    ) -> Result<(), String> {
        tracing::info!(
            "MOCK NOTIFICATION: to={:?}, title={}, body={}",
            tokens,
            title,
            body
        );
        Ok(())
    }
}
