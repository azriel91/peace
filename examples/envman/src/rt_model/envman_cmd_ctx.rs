use peace::{
    cmd::scopes::SingleProfileSingleFlow,
    rt_model::params::{KeyKnown, ParamsKeysImpl},
};

use crate::model::EnvManError;

/// Alias to simplify naming the `CmdCtx` type.
pub type EnvManCmdCtx<'ctx, O, TS> = peace::cmd::ctx::CmdCtx<
    SingleProfileSingleFlow<
        'ctx,
        EnvManError,
        O,
        ParamsKeysImpl<KeyKnown<String>, KeyKnown<String>, KeyKnown<String>>,
        TS,
    >,
>;
