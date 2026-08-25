use sqlx::PgPool;

use crate::config::AppConfig;

pub struct AppState {
    pub postgres_pool: PgPool,
}

impl AppState {
    pub async fn create(app_config: AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            postgres_pool: app_config.database.create_pool().await?,
        })
    }
}
