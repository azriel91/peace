use crate::model::AppCycleError;

/// Alias to simplify naming the CmdContext type.
pub type CmdContext<'ctx, O, TS> =
    peace::rt_model::cmd::CmdContext<'ctx, AppCycleError, O, TS, String, String, ()>;
