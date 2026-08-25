use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvironmentError {
    #[error("missing environment variable: {0}")]
    MissingEnvironment(String),
    #[error("invalid environment variable {name}: {reason}")]
    InvalidEnvironment { name: String, reason: String },
}
