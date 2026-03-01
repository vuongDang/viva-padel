use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub device_id: String,
    pub notif_info: NotifInfo,
}

#[derive(Serialize, Deserialize)]
pub enum NotifInfo {
    Mobile(String),
    Web(web_push::SubscriptionInfo),
}

impl Device {
    pub fn new_mobile(id: &str, token: &str) -> Device {
        Device {
            device_id: id.to_string(),
            notif_info: NotifInfo::Mobile(token.to_string()),
        }
    }

    pub fn new_browser(id: &str, token: web_push::SubscriptionInfo) -> Device {
        Device {
            device_id: id.to_string(),
            notif_info: NotifInfo::Web(token),
        }
    }

    pub fn is_mobile(&self) -> bool {
        matches!(self.notif_info, NotifInfo::Mobile(_))
    }
}
