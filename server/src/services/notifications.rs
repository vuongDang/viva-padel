use std::any::Any;

use async_trait::async_trait;
use serde_json::Value;
use web_push::{IsahcWebPushClient, VapidSignatureBuilder, WebPushClient, WebPushMessageBuilder};

use crate::models::{Device, NotifInfo};

const EXPO_PUSH_API_URL: &str = "https://exp.host/--/api/v2/push/send";
#[async_trait]
pub trait NotificationsService: Send + Sync + Any {
    async fn send_notification(
        &self,
        devices: &[Device],
        title: &str,
        body: &str,
        data: Option<serde_json::Value>,
    ) -> Result<(), String>;
}

pub struct ExpoNotificationsService;

#[async_trait]
impl NotificationsService for ExpoNotificationsService {
    #[tracing::instrument(name = "Sending notification", skip(self, devices, title, data))]
    async fn send_notification(
        &self,
        devices: &[Device],
        title: &str,
        body: &str,
        data: Option<serde_json::Value>,
    ) -> Result<(), String> {
        tracing::error!("STARTING");
        if devices.is_empty() {
            return Ok(());
        }
        tracing::error!("THERE ARE DEVICES");

        let mobile_tokens = devices
            .iter()
            .filter_map(|d| {
                if let NotifInfo::Mobile(ref token) = d.notif_info {
                    Some(token)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let browsers = devices
            .into_iter()
            .filter(|d| !d.is_mobile())
            .collect::<Vec<_>>();

        // Sending to mobile app through Expo push API
        if !mobile_tokens.is_empty() {
            send_mobile_notifications(mobile_tokens, title, body, data.clone()).await?
        }

        if !browsers.is_empty() {
            send_web_notifications(browsers, title, body, data).await?
        }
        Ok(())
    }
}

pub async fn send_mobile_notifications(
    mobile_tokens: Vec<&String>,
    title: &str,
    body: &str,
    data: Option<serde_json::Value>,
) -> Result<(), String> {
    tracing::error!("MOBIIIIIIIIIILE");
    let client = reqwest::Client::new();
    let payload_map = notification_request_payload_for_mobile(mobile_tokens, title, body, data);
    match client
        .post(EXPO_PUSH_API_URL)
        .json(&payload_map)
        .send()
        .await
    {
        Ok(resp) => {
            if !resp.status().is_success() {
                let text = resp.text().await.unwrap_or_default();
                return Err(format!("Expo API error: {}", text));
            } else {
                Ok(())
            }
        }
        Err(e) => return Err(format!("Network error: {}", e)),
    }
}

pub async fn send_web_notifications(
    browsers: Vec<&Device>,
    title: &str,
    body: &str,
    data: Option<serde_json::Value>,
) -> Result<(), String> {
    tracing::error!("WEEEEEEEEEEEEEEEEEEBB");
    let private_key = std::env::var("WEB_PUSH_PRIVATE_KEY").expect("WEB_PUSH_PRIVATE_KEY not set");

    let client = IsahcWebPushClient::new().map_err(|e| e.to_string())?;
    for browser in browsers {
        let NotifInfo::Web(ref info) = browser.notif_info else {
            unreachable!("Only browsers should be in this vector");
        };
        let sig_builder = VapidSignatureBuilder::from_base64(&private_key, info)
            .map_err(|e| e.to_string())?
            .build()
            .map_err(|e| e.to_string())?;

        let payload = serde_json::json!({
            "title": title,
            "body": body,
            "data": data,
            // "icon": "default",
        })
        .to_string();

        let mut builder = WebPushMessageBuilder::new(info);
        builder.set_vapid_signature(sig_builder);
        builder.set_payload(web_push::ContentEncoding::Aes128Gcm, payload.as_bytes());

        client
            .send(builder.build().map_err(|e| e.to_string())?)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn notification_request_payload_for_mobile(
    tokens: Vec<&String>,
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
