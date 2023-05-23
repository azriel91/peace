use serde::{Deserialize, Serialize};

/// Keys for workspace parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum WorkspaceParamsKey {
    /// Default profile to use.
    Profile,
    /// Which flow this workspace is using.
    Flow,
}

/// Keys for profile parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ProfileParamsKey {
    /// Whether the environment is for `Development`, `Production`.
    EnvType,
}
