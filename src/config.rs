use std::time::Duration;

use crate::{
    errors::EnvironmentError,
    infras::{database::DatabaseConfig, env::Environment},
};

#[derive(Debug, Clone)]
pub struct GatewayConfig {
    pub mempool_url: String,
    pub mempool_timeout: Duration
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub gateways: GatewayConfig,
    pub database: DatabaseConfig,
    pub replace_interval: Duration,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, EnvironmentError> {
        Ok(Self {
            host: Environment::as_string("APP_HOST")?,
            port: Environment::parse("APP_PORT")?,
            database: DatabaseConfig::from_env()?,
            gateways: GatewayConfig {
                mempool_url: Environment::as_string("GATEWAY_MEMPOOL_URL")?,
                mempool_timeout: Duration::from_secs(Environment::parse("GATEWAY_MEMPOOL_TIMEOUT_SECONDS")?)
            },
            replace_interval: Duration::from_secs(Environment::parse("REPLACE_INTERVAL_SECONDS")?)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_ok_from_env_when_all_required_variables_are_present() {
        temp_env::with_vars(
        [
                ("APP_HOST", Some("0.0.0.0")),
                ("APP_PORT", Some("8080")),
                (
                    "GATEWAY_MEMPOOL_URL",
                    Some("https://mempool.space/api/v1/lightning/nodes/rankings/connectivity"),
                ),
                (
                    "GATEWAY_MEMPOOL_TIMEOUT_SECONDS",
                    Some("5"),
                ),
                (
                    "DATABASE_URL",
                    Some("postgres://postgres:postgres@localhost:5432/lightning_nodes"),
                ),
                (
                    "DATABASE_MIN_CONNECTIONS",
                    Some("1"),
                ),
                (
                    "DATABASE_MAX_CONNECTIONS",
                    Some("10"),
                ),
                 (
                    "REPLACE_INTERVAL_SECONDS",
                    Some("900"),
                ),
            ],
            || {
                let result = AppConfig::from_env();

                assert!(result.is_ok());

                let config = result.unwrap();

                assert_eq!(config.host, "0.0.0.0");
                assert_eq!(config.port, 8080);
                assert_eq!(
                    config.gateways.mempool_url,
                    "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity"
                );
            },
        );
    }

    #[test]
    fn should_return_error_from_env_when_app_host_is_missing() {
        temp_env::with_vars(
            [
                ("APP_HOST", None),
                ("APP_PORT", Some("8080")),
            ],
            || {
                let result = AppConfig::from_env();

                assert!(matches!(
                    result,
                    Err(EnvironmentError::MissingEnvironment(name))
                        if name == "APP_HOST"
                ));
            },
        );
    }

    #[test]
    fn should_return_error_when_app_port_is_missing() {
        temp_env::with_vars(
            [
                ("APP_HOST", Some("0.0.0.0")),
                ("APP_PORT", None),
            ],
            || {
                let result = AppConfig::from_env();

                assert!(matches!(
                    result,
                    Err(EnvironmentError::MissingEnvironment(name))
                        if name == "APP_PORT"
                ));
            },
        );
    }

    #[test]
    fn should_return_error_from_env_when_mempool_url_is_missing() {
        temp_env::with_vars(
            [
                ("APP_HOST", Some("0.0.0.0")),
                ("APP_PORT", Some("8080")),
                (
                    "GATEWAY_MEMPOOL_URL",
                    None,
                ),
                (
                    "DATABASE_URL",
                    Some("postgres://postgres:postgres@localhost:5432/lightning_nodes"),
                ),
                (
                    "DATABASE_MIN_CONNECTIONS",
                    Some("1"),
                ),
                (
                    "DATABASE_MAX_CONNECTIONS",
                    Some("10"),
                ),
            ],
            || {
                let result = AppConfig::from_env();

                assert!(matches!(
                    result,
                    Err(EnvironmentError::MissingEnvironment(name))
                        if name == "GATEWAY_MEMPOOL_URL"
                ));
            },
        );
    }

    #[test]
    fn should_return_error_from_env_when_mempool_timeour_is_missing() {
        temp_env::with_vars(
            [
                ("APP_HOST", Some("0.0.0.0")),
                ("APP_PORT", Some("8080")),
                (
                    "GATEWAY_MEMPOOL_URL",
                    Some("https://mempool.space/api/v1/lightning/nodes/rankings/connectivity"),
                ),
                (
                    "GATEWAY_MEMPOOL_TIMEOUT_SECONDS",
                    None,
                ),
                (
                    "DATABASE_URL",
                    Some("postgres://postgres:postgres@localhost:5432/lightning_nodes"),
                ),
                (
                    "DATABASE_MIN_CONNECTIONS",
                    Some("1"),
                ),
                (
                    "DATABASE_MAX_CONNECTIONS",
                    Some("10"),
                ),
                 (
                    "REPLACE_INTERVAL_SECONDS",
                    Some("900"),
                ),
            ],
            || {
                let result = AppConfig::from_env();

                assert!(matches!(
                    result,
                    Err(EnvironmentError::MissingEnvironment(name))
                        if name == "GATEWAY_MEMPOOL_TIMEOUT_SECONDS"
                ));
            },
        );
    }

    #[test]
    fn should_return_error_from_env_when_database_url_is_missing() {
        temp_env::with_vars(
            [
                ("APP_HOST", Some("0.0.0.0")),
                ("APP_PORT", Some("8080")),
                (
                    "GATEWAY_MEMPOOL_URL",
                    Some("https://mempool.space/api/v1/lightning/nodes/rankings/connectivity"),
                ),
                ("DATABASE_URL", None),
            ],
            || {
                let result = AppConfig::from_env();

                assert!(matches!(
                    result,
                    Err(EnvironmentError::MissingEnvironment(name))
                        if name == "DATABASE_URL"
                ));
            },
        );
    }

    #[test]
    fn should_return_error_from_env_when_replace_interval_is_missing() {
        temp_env::with_vars(
            [
                ("APP_HOST", Some("0.0.0.0")),
                ("APP_PORT", Some("8080")),
                (
                    "GATEWAY_MEMPOOL_URL",
                    Some("https://mempool.space/api/v1/lightning/nodes/rankings/connectivity"),
                ),
                (
                    "GATEWAY_MEMPOOL_TIMEOUT_SECONDS",
                    Some("5"),
                ),
                (
                    "DATABASE_URL",
                    Some("postgres://postgres:postgres@localhost:5432/lightning_nodes"),
                ),
                (
                    "DATABASE_MIN_CONNECTIONS",
                    Some("1"),
                ),
                (
                    "DATABASE_MAX_CONNECTIONS",
                    Some("10"),
                ),
                 (
                    "REPLACE_INTERVAL_SECONDS",
                    None,
                ),
            ],
            || {
                let result = AppConfig::from_env();

                assert!(matches!(
                    result,
                    Err(EnvironmentError::MissingEnvironment(name))
                        if name == "REPLACE_INTERVAL_SECONDS"
                ));
            },
        );
    }
}