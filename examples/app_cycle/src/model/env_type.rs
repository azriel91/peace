use std::str::FromStr;

use peace::fmt::Presenter;
use serde::{Deserialize, Serialize};

use crate::model::EnvTypeParseError;

/// Type of environment: development or production.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
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

impl std::fmt::Display for EnvType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvType::Development => "development".fmt(f),
            EnvType::Production => "production".fmt(f),
        }
    }
}

#[peace::fmt::async_trait(?Send)]
impl peace::fmt::Presentable for EnvType {
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        match self {
            EnvType::Development => presenter.code_inline("development").await,
            EnvType::Production => presenter.code_inline("production").await,
        }
    }
}
