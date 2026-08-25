use std::env;

use crate::errors::EnvironmentError;

pub struct Environment;
impl Environment {
    pub fn as_string(name: &str) -> Result<String, EnvironmentError> {
        env::var(name).map_err(|_| EnvironmentError::MissingEnvironment(name.to_owned()))
    }

    pub fn parse<T: std::str::FromStr>(name: &str) -> Result<T, EnvironmentError>
    where
        T::Err: std::fmt::Display,
    {
        let value = Self::as_string(name)?;
        value
            .parse::<T>()
            .map_err(|error| EnvironmentError::InvalidEnvironment {
                name: name.to_owned(),
                reason: error.to_string(),
            })
    }
}
