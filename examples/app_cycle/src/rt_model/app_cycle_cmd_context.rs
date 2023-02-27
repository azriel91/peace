use peace::rt_model::cmd_context_params::{KeyKnown, KeyUnknown, ParamsKeysImpl};

use crate::model::AppCycleError;

/// Alias to simplify naming the CmdContext type.
pub type AppCycleCmdContext<'ctx, O, TS> = peace::rt_model::cmd::CmdContext<
    'ctx,
    AppCycleError,
    O,
    TS,
    ParamsKeysImpl<KeyKnown<String>, KeyKnown<String>, KeyUnknown>,
>;
