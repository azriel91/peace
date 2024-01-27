use std::pin::Pin;

use crate::CmdBlockRt;

/// Alias for `Box<dyn CmdBlockRt<..>>`.
///
/// # Type Parameters
///
/// * `AppErrorT`: Automation software error type.
/// * `ParamsKeysT`: Types of params keys.
/// * `Outcome`: [`CmdBlock`] outcome type, e.g. `(StatesCurrent, StatesGoal)`.
///
/// [`CmdBlock`]: crate::CmdBlock
pub type CmdBlockRtBox<'ctx, AppErrorT, ParamsKeysT, ExecutionOutcome> = Pin<
    Box<
        dyn CmdBlockRt<
                AppError = AppErrorT,
                ParamsKeys = ParamsKeysT,
                ExecutionOutcome = ExecutionOutcome,
            > + 'ctx,
    >,
>;
