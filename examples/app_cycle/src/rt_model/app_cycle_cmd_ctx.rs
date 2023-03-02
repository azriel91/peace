use peace::{
    cmd::scopes::SingleProfileSingleFlow,
    rt_model::params::{KeyKnown, KeyUnknown, ParamsKeysImpl},
};

use crate::model::AppCycleError;

/// Alias to simplify naming the `CmdCtx` type.
pub type AppCycleCmdCtx<'ctx, O, TS> = peace::cmd::ctx::CmdCtx<
    'ctx,
    O,
    SingleProfileSingleFlow<
        AppCycleError,
        ParamsKeysImpl<KeyKnown<String>, KeyKnown<String>, KeyUnknown>,
        TS,
    >,
>;
