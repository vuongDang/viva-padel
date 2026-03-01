use tokio::sync::Mutex;

use async_trait::async_trait;
use serde_json::Value;

use crate::{
    models::Device,
    services::{NotificationsService, notifications::notification_request_payload_for_mobile},
};

#[derive(Default)]
pub struct MockNotificationsService {
    pub notifications: Mutex<Vec<Value>>,
}

#[async_trait]
impl NotificationsService for MockNotificationsService {
    #[tracing::instrument(name = "Sending notification", skip(self, devices, title, data))]
    async fn send_notification(
        &self,
        devices: &[Device],
        title: &str,
        body: &str,
        data: Option<serde_json::Value>,
    ) -> Result<(), String> {
        let mobile_tokens = devices
            .iter()
            .filter_map(|d| {
                if let crate::models::NotifInfo::Mobile(ref token) = d.notif_info {
                    Some(token)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if !mobile_tokens.is_empty() {
            let notif = notification_request_payload_for_mobile(mobile_tokens, title, body, data);
            self.notifications.lock().await.push(notif);
        }
        Ok(())
    }
}
