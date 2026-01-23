use crate::services::{DataBaseService, database::DBError};
use async_trait::async_trait;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub struct MockDB {
    pool: sqlx::SqlitePool,
}

#[async_trait]
impl DataBaseService for MockDB {
    fn get_db_pool(&self) -> &SqlitePool {
        &self.pool
    }
}

impl MockDB {
    pub async fn new() -> Result<Self, DBError> {
        dotenvy::dotenv().map_err(|e| DBError::Env(e.to_string()))?;

        let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await?;

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| DBError::Sqlx(e.into()))?;
        Ok(MockDB { pool })
    }
}
