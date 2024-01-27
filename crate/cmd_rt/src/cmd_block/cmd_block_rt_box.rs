use std::pin::Pin;

use crate::CmdBlockRt;

/// Alias for `Box<dyn CmdBlockRt<..>>`.
///
/// # Type Parameters
///
/// * `E`: Automation software error type.
/// * `PKeys`: Types of params keys.
/// * `Outcome`: [`CmdBlock`] outcome type, e.g. `(StatesCurrent, StatesGoal)`.
///
/// [`CmdBlock`]: crate::CmdBlock
pub type CmdBlockRtBox<'ctx, CmdCtxTypeParamsT, ExecutionOutcome> = Pin<
    Box<
        dyn CmdBlockRt<CmdCtxTypeParams = CmdCtxTypeParamsT, ExecutionOutcome = ExecutionOutcome>
            + 'ctx,
    >,
>;
