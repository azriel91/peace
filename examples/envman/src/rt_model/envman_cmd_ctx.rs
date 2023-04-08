use peace::{
    cmd::scopes::SingleProfileSingleFlow,
    rt_model::params::{KeyKnown, ParamsKeysImpl},
};

use crate::model::{EnvDeployFlowParamsKey, EnvManError, ProfileParamsKey, WorkspaceParamsKey};

/// Alias to simplify naming the `CmdCtx` type.
pub type EnvManCmdCtx<'ctx, O, TS> = peace::cmd::ctx::CmdCtx<
    SingleProfileSingleFlow<
        'ctx,
        EnvManError,
        O,
        ParamsKeysImpl<
            KeyKnown<WorkspaceParamsKey>,
            KeyKnown<ProfileParamsKey>,
            KeyKnown<EnvDeployFlowParamsKey>,
        >,
        TS,
    >,
>;
