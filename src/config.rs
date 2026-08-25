use crate::{
    errors::EnvironmentError,
    infras::{database::DatabaseConfig, env::Environment},
};

#[derive(Debug, Clone)]
pub struct GatewayConfig {
    pub mempool_url: String,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub gateways: GatewayConfig,
    pub database: DatabaseConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, EnvironmentError> {
        dotenvy::dotenv().ok();
        Ok(Self {
            host: Environment::as_string("APP_HOST")?,
            port: Environment::parse("APP_PORT")?,
            database: DatabaseConfig::from_env()?,
            gateways: GatewayConfig {
                mempool_url: Environment::as_string("GATEWAY_MEMPOOL_URL")?,
            },
        })
    }
}
