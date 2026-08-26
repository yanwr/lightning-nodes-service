use std::sync::Arc;

use sqlx::PgPool;

use crate::{config::AppConfig, errors::AppError, gateways::mempool::Gateways};

#[derive(Debug, Clone)]
pub struct AppState {
    pub postgres_pool: PgPool,
    pub gateways: Arc<Gateways>,
}

impl AppState {
    pub async fn create(app_config: AppConfig) -> Result<Self, AppError> {
        Ok(Self {
            gateways: Arc::new(Gateways::new(&app_config)?),
            postgres_pool: app_config.database.create_pool().await?,
        })
    }
}
