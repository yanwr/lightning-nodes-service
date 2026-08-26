use sqlx::PgPool;

use crate::{config::AppConfig, gateways::mempool::Gateways};

pub struct AppState {
    pub postgres_pool: PgPool,
    pub gateways: Gateways,
}

impl AppState {
    pub async fn create(app_config: AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            gateways: Gateways::new(&app_config)?,
            postgres_pool: app_config.database.create_pool().await?,
        })
    }
}
