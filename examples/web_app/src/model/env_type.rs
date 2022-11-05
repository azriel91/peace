use std::str::FromStr;

use crate::model::EnvTypeParseError;

/// Type of environment: development or production.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnvType {
    /// Development environment that runs on `localhost`.
    Development,
    /// Environment that runs in AWS.
    ///
    /// Credentials are read from the environment.
    Production,
}

impl FromStr for EnvType {
    type Err = EnvTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            _ => Err(EnvTypeParseError(s.to_string())),
        }
    }
}
