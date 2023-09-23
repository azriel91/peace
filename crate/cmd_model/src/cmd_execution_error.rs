use std::fmt::Debug;

use crate::CmdBlockDesc;

/// Error while executing a `CmdBlock`.
///
/// # Type Parameters
///
/// * `E`: Application error type.
#[cfg_attr(feature = "error_reporting", derive(miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum CmdExecutionError {
    /// Error fetching `CmdBlock::InputT` from `resources`.
    ///
    /// If `CmdBlock::InputT` is a tuple, such as `(StatesCurrent, StatesGoal)`,
    /// and `states_current` and `states_goal` are inserted individually in
    /// `Resources`, then `CmdBlock::input_fetch` should be implemented to call
    /// `Resources::remove` for each of them.
    #[error(
        "Error in `CmdExecution` or `CmdBlock` logic, usually due to incorrect `Resource` insertion or removal."
    )]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure that the value is populated by a predecessor."))
    )]
    InputFetch {
        /// String representations of the `CmdBlock`s in this `CmdExecution`.
        ///
        /// These are used to provide a well-formatted error message so that
        /// developers can identify where a bug lies more easily.
        cmd_block_descs: Vec<CmdBlockDesc>,
        /// Index of the `CmdBlock` for which `input_fetch` failed.
        cmd_block_index: usize,
        /// Short type name of the `CmdBlock::Input` type.
        input_name_short: String,
        /// Full type name of the `CmdBlock::Input` type.
        input_name_full: String,
    },
}
