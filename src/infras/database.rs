use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::{errors::EnvironmentError, infras::env::Environment};

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, EnvironmentError> {
        Ok(Self {
            url: Environment::as_string("DATABASE_URL")?,
            max_connections: Environment::parse("DATABASE_MAX_CONNECTIONS")?,
            min_connections: Environment::parse("DATABASE_MIN_CONNECTIONS")?,
        })
    }

    pub async fn create_pool(self) -> Result<Pool<Postgres>, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .min_connections(self.min_connections)
            .max_connections(self.max_connections)
            .test_before_acquire(true)
            .connect(&self.url)
            .await?;
        Ok(pool)
    }
}
