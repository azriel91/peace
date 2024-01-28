use peace::{
    cmd::ctx::CmdCtxTypesCollector,
    rt_model::params::{KeyKnown, KeyUnknown, ParamsKeysImpl},
};

use crate::model::{EnvManError, ProfileParamsKey, WorkspaceParamsKey};

pub type EnvmanCmdCtxTypes<Output> = CmdCtxTypesCollector<
    EnvManError,
    Output,
    ParamsKeysImpl<KeyKnown<WorkspaceParamsKey>, KeyKnown<ProfileParamsKey>, KeyUnknown>,
>;
