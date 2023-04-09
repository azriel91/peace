use serde::{Deserialize, Serialize};

/// Keys for workspace parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum WorkspaceParamsKey {
    /// Default profile to use.
    Profile,
}

/// Keys for profile parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ProfileParamsKey {
    /// Whether the environment is for `Development`, `Production`.
    EnvType,
}

/// Keys for the environment deploy flow parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum EnvDeployFlowParamsKey {
    /// Parameters to download the web app.
    AppDownloadParams,
    /// Parameters to extract the downloaded web app.
    AppExtractParams,
    /// Parameters to create the IAM policy.
    IamPolicyParams,
    /// Parameters to create the IAM role to add to the instance profile.
    IamRoleParams,
    /// Parameters to create the instance profile to assign to the EC2 instance.
    InstanceProfileParams,
    /// Parameters to create S3 bucket.
    S3BucketParams,
    /// Parameters to upload the web app as an S3 object.
    S3ObjectParams,
}
