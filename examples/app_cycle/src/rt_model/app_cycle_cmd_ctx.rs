use peace::{
    cmd::scopes::SingleProfileSingleFlow,
    rt_model::params::{KeyKnown, ParamsKeysImpl},
};

use crate::model::AppCycleError;

/// Alias to simplify naming the `CmdCtx` type.
pub type AppCycleCmdCtx<'ctx, O, TS> = peace::cmd::ctx::CmdCtx<
    SingleProfileSingleFlow<
        'ctx,
        AppCycleError,
        O,
        ParamsKeysImpl<KeyKnown<String>, KeyKnown<String>, KeyKnown<String>>,
        TS,
    >,
>;
