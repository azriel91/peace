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
    /// If `CmdBlock::InputT` is a tuple, such as `(StatesCurrent<ItemIdT>,
    /// StatesGoal<ItemIdT>)`, and `states_current` and `states_goal` are
    /// inserted individually in `Resources`, then `CmdBlock::input_fetch`
    /// should be implemented to call `Resources::remove` for each of them.
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
        /// Textual representation of the `CmdExecution`.
        ///
        /// This includes the `CmdBlock`s and their `InputT` and `Outcome` type
        /// names.
        ///
        /// Approximation of the source for `EnsureCmd`:
        ///
        /// ```yaml
        /// CmdExecution:
        ///   ExecutionOutcome: (States<ItemIdT, Previous>, States<ItemIdT, Ensured>, States<ItemIdT, Goal>)
        /// CmdBlocks:
        ///   - StatesCurrentReadCmdBlock:
        ///       Input: States<ItemIdT, Current>
        ///       Outcome: States<ItemIdT, Goal>
        ///   - StatesGoalReadCmdBlock:
        ///       Input: States<ItemIdT, Current>
        ///       Outcome: States<ItemIdT, Goal>
        ///   - StatesDiscoverCmdBlock:
        ///       Input: ()
        ///       Outcome: (States<ItemIdT, Current>, States<ItemIdT, Goal>)
        ///   - ApplyStateSyncCheckCmdBlock:
        ///       Input: (States<ItemIdT, CurrentStored>, States<ItemIdT, Current>, States<ItemIdT, GoalStored>, States<ItemIdT, Goal>)
        ///       Outcome: (States<ItemIdT, CurrentStored>, States<ItemIdT, Current>, States<ItemIdT, GoalStored>, States<ItemIdT, Goal>)
        ///   - ApplyExecCmdBlock:
        ///       Input: (States<ItemIdT, Current>, States<ItemIdT, Goal>)
        ///       Outcome: (States<ItemIdT, Previous>, States<ItemIdT, Ensured>, States<ItemIdT, Goal>)
        /// ```
        #[cfg(feature = "error_reporting")]
        #[source_code]
        cmd_execution_src: String,
        /// Span within the source text of the input type.
        #[cfg(feature = "error_reporting")]
        #[label("This type is not present in `resources`")]
        input_span: Option<miette::SourceSpan>,
        /// Full span so that miette renders the whole `cmd_execution_src`.
        #[cfg(feature = "error_reporting")]
        #[label]
        full_span: miette::SourceSpan,
    },
}
