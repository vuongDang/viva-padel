use crate::models::{Alarm, User};
use async_trait::async_trait;
use sqlx::SqlitePool;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("User already exists: {0}")]
    UserAlreadyExists(String),
    #[error("User not found")]
    UserNotFound,
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Environment error: {0}")]
    Env(String),
}

#[async_trait]
pub trait DataBaseService: Send + Sync {
    async fn create_user(&self, email: &str) -> Result<User, DBError> {
        let user = User::new(email.to_owned());

        let user_already_exists = self.get_user_by_email(email).await.is_ok();
        if user_already_exists {
            return Err(DBError::UserAlreadyExists(email.to_string()));
        }

        let email_clone = user.email.clone();
        sqlx::query!(
            "INSERT INTO users (id, email, created_at) VALUES (?, ?, ?)",
            user.id,
            email_clone,
            user.created_at
        )
        .execute(self.get_db_pool())
        .await?;
        Ok(user)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<User, DBError> {
        sqlx::query_as::<_, User>("SELECT * FROM users where id = ?")
            .bind(id)
            .fetch_one(self.get_db_pool())
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => DBError::UserNotFound,
                _ => DBError::Sqlx(e),
            })
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, DBError> {
        sqlx::query_as::<_, User>("SELECT * FROM users where email = ?")
            .bind(email)
            .fetch_one(self.get_db_pool())
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => DBError::UserNotFound,
                _ => DBError::Sqlx(e),
            })
    }

    async fn register_device(
        &self,
        device_id: &str,
        notif_token: &str,
        user_id: Uuid,
    ) -> Result<(), DBError> {
        sqlx::query!(
            "INSERT INTO devices (device_id, notif_token, user_id, last_seen) VALUES (?, ?, ?, CURRENT_TIMESTAMP)
            ON CONFLICT(device_id)
            DO UPDATE SET
            user_id = excluded.user_id,
            last_seen = CURRENT_TIMESTAMP,
            notif_token = excluded.notif_token",
            device_id,
            notif_token,
            user_id
        )
        .execute(self.get_db_pool())
        .await?;
        Ok(())
    }

    async fn get_devices_for_user(&self, user_id: Uuid) -> Result<Vec<String>, DBError> {
        let rows = sqlx::query!(
            r#"SELECT device_id FROM devices WHERE user_id = ?"#,
            user_id
        )
        .fetch_all(self.get_db_pool())
        .await?;
        Ok(rows.into_iter().filter_map(|r| r.device_id).collect())
    }

    async fn update_alarms(&self, user_id: Uuid, alarms: Vec<Alarm>) -> Result<(), DBError> {
        let mut tx = self.get_db_pool().begin().await?;

        sqlx::query!("DELETE FROM alarms WHERE user_id = ?", user_id)
            .execute(&mut *tx)
            .await?;

        let now = chrono::Utc::now();
        for alarm in alarms {
            let alarm_id = Uuid::new_v4();
            let alarm_json = serde_json::json!(alarm).to_string();
            sqlx::query!(
                "INSERT INTO alarms (id, user_id, alarm_json, is_active, created_at) VALUES (?, ?, ?, ?, ?)",
                alarm_id,
                user_id,
                alarm_json,
                alarm.is_active,
                now
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn get_user_alarms(&self, user_id: Uuid) -> Result<Vec<Alarm>, DBError> {
        let rows = sqlx::query!("SELECT alarm_json FROM alarms WHERE user_id = ?", user_id)
            .fetch_all(self.get_db_pool())
            .await?;

        let alarms = rows
            .into_iter()
            .map(|record| {
                serde_json::from_str::<Alarm>(&record.alarm_json).map_err(DBError::Serialization)
            })
            .collect::<Result<Vec<Alarm>, DBError>>()?;
        Ok(alarms)
    }
    async fn get_active_alarms(&self) -> Result<Vec<(Uuid, Alarm)>, DBError> {
        let rows = sqlx::query!(
            r#"
                SELECT user_id as "user_id: Uuid", alarm_json
                FROM alarms
                WHERE is_active is TRUE
            "#,
        )
        .fetch_all(self.get_db_pool())
        .await?;

        let alarms = rows
            .into_iter()
            .filter_map(|row| {
                let alarm: Alarm = serde_json::from_str(&row.alarm_json).ok()?;
                Some((row.user_id, alarm))
            })
            .collect();
        Ok(alarms)
    }

    async fn get_tokens_for_user(&self, user_id: Uuid) -> Result<Vec<String>, DBError> {
        let rows = sqlx::query!(
            r#"SELECT notif_token FROM devices WHERE user_id = ?"#,
            user_id
        )
        .fetch_all(self.get_db_pool())
        .await?;
        Ok(rows.into_iter().map(|r| r.notif_token).collect())
    }
    fn get_db_pool(&self) -> &SqlitePool;
}

pub struct SQLiteDB {
    pub pool: sqlx::SqlitePool,
}

#[async_trait]
impl DataBaseService for SQLiteDB {
    fn get_db_pool(&self) -> &SqlitePool {
        &self.pool
    }
}

impl SQLiteDB {
    pub async fn new() -> Result<Self, DBError> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| DBError::Env("DATABASE_URL not set".into()))?;
        tracing::info!("db url: {:?}", &database_url);
        let pool = SqlitePool::connect(&database_url).await?;

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| DBError::Sqlx(e.into()))?;
        Ok(SQLiteDB { pool })
    }
}
