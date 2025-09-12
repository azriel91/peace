use peace::{
    cmd_ctx::type_reg::untagged::TypeReg, enum_iterator::Sequence, params::ParamsKey,
    profile_model::Profile,
};
use serde::{Deserialize, Serialize};

use crate::model::{EnvManFlow, EnvType};

/// Keys for workspace parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Sequence)]
#[enum_iterator(crate = peace::enum_iterator)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceParamsKey {
    /// Default profile to use.
    Profile,
    /// Which flow this workspace is using.
    Flow,
}

impl ParamsKey for WorkspaceParamsKey {
    fn register_value_type(self, type_reg: &mut TypeReg<Self>) {
        match self {
            Self::Profile => type_reg.register::<Profile>(self),
            Self::Flow => type_reg.register::<EnvManFlow>(self),
        }
    }
}

/// Keys for profile parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Sequence)]
#[enum_iterator(crate = peace::enum_iterator)]
#[serde(rename_all = "snake_case")]
pub enum ProfileParamsKey {
    /// Whether the environment is for `Development`, `Production`.
    EnvType,
}

impl ParamsKey for ProfileParamsKey {
    fn register_value_type(self, type_reg: &mut TypeReg<Self>) {
        match self {
            Self::EnvType => type_reg.register::<EnvType>(self),
        }
    }
}
