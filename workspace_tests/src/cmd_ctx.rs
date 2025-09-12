use peace::{
    cmd_ctx::type_reg::untagged::TypeReg, enum_iterator::Sequence, params::ParamsKey,
    profile_model::Profile,
};
use serde::{Deserialize, Serialize};

mod cmd_ctx_mpnf;
mod cmd_ctx_mpnf_params;
mod cmd_ctx_mpsf;
mod cmd_ctx_mpsf_params;
mod cmd_ctx_npnf;
mod cmd_ctx_npnf_params;
mod cmd_ctx_spnf;
mod cmd_ctx_spnf_params;
mod cmd_ctx_spsf;
mod cmd_ctx_spsf_params;

/// Keys for workspace parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Sequence)]
#[enum_iterator(crate = peace::enum_iterator)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceParamsKey {
    Profile,
    StringParam,
    U8Param,
}

impl ParamsKey for WorkspaceParamsKey {
    fn register_value_type(self, type_reg: &mut TypeReg<Self>) {
        match self {
            Self::Profile => type_reg.register::<Profile>(self),
            Self::StringParam => type_reg.register::<String>(self),
            Self::U8Param => type_reg.register::<u8>(self),
        }
    }
}

/// Keys for profile parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Sequence)]
#[enum_iterator(crate = peace::enum_iterator)]
#[serde(rename_all = "snake_case")]
pub enum ProfileParamsKey {
    U32Param,
    U64Param,
    I64Param,
}

impl ParamsKey for ProfileParamsKey {
    fn register_value_type(self, type_reg: &mut TypeReg<Self>) {
        match self {
            Self::U32Param => type_reg.register::<u32>(self),
            Self::U64Param => type_reg.register::<u64>(self),
            Self::I64Param => type_reg.register::<i64>(self),
        }
    }
}

/// Keys for flow parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Sequence)]
#[enum_iterator(crate = peace::enum_iterator)]
#[serde(rename_all = "snake_case")]
pub enum FlowParamsKey {
    BoolParam,
    U16Param,
    I16Param,
}

impl ParamsKey for FlowParamsKey {
    fn register_value_type(self, type_reg: &mut TypeReg<Self>) {
        match self {
            Self::BoolParam => type_reg.register::<bool>(self),
            Self::U16Param => type_reg.register::<u16>(self),
            Self::I16Param => type_reg.register::<i16>(self),
        }
    }
}
