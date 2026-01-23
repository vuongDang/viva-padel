use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub created_at: i64,
}

impl User {
    pub fn new(email: String) -> Self {
        User {
            id: uuid::Uuid::new_v4(),
            email,
            created_at: Utc::now().timestamp(),
        }
    }
}
