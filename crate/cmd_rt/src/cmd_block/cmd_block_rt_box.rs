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
pub type CmdBlockRtBox<E, PKeys> = Pin<Box<dyn CmdBlockRt<Error = E, PKeys = PKeys>>>;
