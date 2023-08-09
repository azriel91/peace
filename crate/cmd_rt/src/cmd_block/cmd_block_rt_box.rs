use std::pin::Pin;

use crate::CmdBlockRt;

/// Alias for `Box<dyn CmdBlockRt<..>>`.
///
/// # Type Parameters
///
/// * `E`: Automation software error type.
/// * `PKeys`: Types of params keys.
/// * `OutcomeT`: [`CmdBlock`] outcome type, e.g. `(StatesCurrent, StatesGoal)`.
///
/// [`CmdBlock`]: crate::CmdBlock
pub type CmdBlockRtBox<E, PKeys, OutcomeT> =
    Pin<Box<dyn CmdBlockRt<Error = E, PKeys = PKeys, OutcomeT = OutcomeT>>>;
