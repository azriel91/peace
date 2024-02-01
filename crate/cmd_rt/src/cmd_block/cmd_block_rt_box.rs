use std::pin::Pin;

use crate::CmdBlockRt;

/// Alias for `Box<dyn CmdBlockRt<..>>`.
///
/// # Type Parameters
///
/// * `E`: Automation software error type.
/// * `PKeys`: Types of params keys.
/// * `Outcome`: [`CmdBlock`] outcome type, e.g. `(StatesCurrent<ItemIdT>,
///   StatesGoal<ItemIdT>)`.
///
/// [`CmdBlock`]: crate::CmdBlock
pub type CmdBlockRtBox<'types, CmdCtxTypesT, ExecutionOutcome> = Pin<
    Box<dyn CmdBlockRt<CmdCtxTypes = CmdCtxTypesT, ExecutionOutcome = ExecutionOutcome> + 'types>,
>;
