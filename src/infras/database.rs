use sqlx::{PgPool, Pool, Postgres, migrate::MigrateError, postgres::PgPoolOptions};

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
        Self::run_migrate(&pool).await?;
        Ok(pool)
    }

    async fn run_migrate(pool: &PgPool) -> Result<(), MigrateError> {
        sqlx::migrate!("./migrations").run(pool).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_ok_from_env_when_all_required_variables_are_present() {
        temp_env::with_vars(
        [
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
                let result = DatabaseConfig::from_env();

                assert!(result.is_ok());

                let config = result.unwrap();

                assert_eq!(config.max_connections, 10);
                assert_eq!(config.min_connections, 1);
                assert_eq!(
                    config.url,
                    "postgres://postgres:postgres@localhost:5432/lightning_nodes"
                );
            },
        );
    }

    #[test]
    fn should_return_error_from_env_when_db_url_is_missing() {
        temp_env::with_vars(
            [
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
                let result = DatabaseConfig::from_env();

                assert!(matches!(
                    result,
                    Err(EnvironmentError::MissingEnvironment(name))
                        if name == "DATABASE_URL"
                ));
            },
        );
    }

    #[test]
    fn should_return_error_from_env_when_db_max_is_missing() {
        temp_env::with_vars(
            [
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
                    None,
                ),
            ],
            || {
                let result = DatabaseConfig::from_env();

                assert!(matches!(
                    result,
                    Err(EnvironmentError::MissingEnvironment(name))
                        if name == "DATABASE_MAX_CONNECTIONS"
                ));
            },
        );
    }

    #[test]
    fn should_return_error_from_env_when_db_min_is_missing() {
        temp_env::with_vars(
            [
                (
                    "DATABASE_URL",
                    Some("postgres://postgres:postgres@localhost:5432/lightning_nodes"),
                ),
                (
                    "DATABASE_MIN_CONNECTIONS",
                    None,
                ),
                (
                    "DATABASE_MAX_CONNECTIONS",
                    Some("10"),
                ),
            ],
            || {
                let result = DatabaseConfig::from_env();

                assert!(matches!(
                    result,
                    Err(EnvironmentError::MissingEnvironment(name))
                        if name == "DATABASE_MIN_CONNECTIONS"
                ));
            },
        );
    }
}
