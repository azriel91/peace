use std::str::FromStr;

use peace::fmt::Presenter;
use serde::{Deserialize, Serialize};

use crate::model::EnvManFlowParseError;

/// Which flow to use: application upload or environment deploy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum EnvManFlow {
    /// Upload the application to S3.
    AppUpload,
    /// Deploy the full environment.
    #[default]
    EnvDeploy,
}

impl FromStr for EnvManFlow {
    type Err = EnvManFlowParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "upload" => Ok(Self::AppUpload),
            "deploy" => Ok(Self::EnvDeploy),
            _ => Err(EnvManFlowParseError(s.to_string())),
        }
    }
}

impl std::fmt::Display for EnvManFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvManFlow::AppUpload => "upload".fmt(f),
            EnvManFlow::EnvDeploy => "deploy".fmt(f),
        }
    }
}

#[peace::fmt::async_trait(?Send)]
impl peace::fmt::Presentable for EnvManFlow {
    async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
    where
        PR: Presenter<'output>,
    {
        match self {
            EnvManFlow::AppUpload => presenter.code_inline("upload").await,
            EnvManFlow::EnvDeploy => presenter.code_inline("deploy").await,
        }
    }
}
