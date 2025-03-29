use std::borrow::Borrow;

use crate::CmdBlockDesc;

/// Error fetching `CmdBlock::InputT` from `resources`.
///
/// If `CmdBlock::InputT` is a tuple, such as `(StatesCurrent, StatesGoal)`,
/// and `states_current` and `states_goal` are inserted individually in
/// `Resources`, then `CmdBlock::input_fetch` should be implemented to call
/// `Resources::remove` for each of them.
#[derive(Debug, thiserror::Error)]
#[error(
    "Error in `CmdExecution` or `CmdBlock` logic, usually due to incorrect `Resource` insertion or removal."
)]
#[cfg_attr(
    feature = "error_reporting",
    derive(miette::Diagnostic),
    diagnostic(help("Make sure that the value is populated by a predecessor."))
)]
pub struct InputFetchError {
    /// String representations of the `CmdBlock`s in this `CmdExecution`.
    ///
    /// These are used to provide a well-formatted error message so that
    /// developers can identify where a bug lies more easily.
    pub cmd_block_descs: Vec<CmdBlockDesc>,
    /// Index of the `CmdBlock` for which `input_fetch` failed.
    pub cmd_block_index: usize,
    /// Short type name of the `CmdBlock::Input` type.
    pub input_name_short: String,
    /// Full type name of the `CmdBlock::Input` type.
    pub input_name_full: String,
    /// Textual representation of the `CmdExecution`.
    ///
    /// This includes the `CmdBlock`s and their `InputT` and `Outcome` type
    /// names.
    ///
    /// Approximation of the source for `EnsureCmd`:
    ///
    /// ```yaml
    /// CmdExecution:
    ///   ExecutionOutcome: (States<Previous>, States<Ensured>, States<Goal>)
    /// CmdBlocks:
    ///   - StatesCurrentReadCmdBlock:
    ///       Input: States<Current>
    ///       Outcome: States<Goal>
    ///   - StatesGoalReadCmdBlock:
    ///       Input: States<Current>
    ///       Outcome: States<Goal>
    ///   - StatesDiscoverCmdBlock:
    ///       Input: ()
    ///       Outcome: (States<Current>, States<Goal>)
    ///   - ApplyStateSyncCheckCmdBlock:
    ///       Input: (States<CurrentStored>, States<Current>, States<GoalStored>, States<Goal>)
    ///       Outcome: (States<CurrentStored>, States<Current>, States<GoalStored>, States<Goal>)
    ///   - ApplyExecCmdBlock:
    ///       Input: (States<Current>, States<Goal>)
    ///       Outcome: (States<Previous>, States<Ensured>, States<Goal>)
    /// ```
    #[cfg(feature = "error_reporting")]
    #[source_code]
    pub cmd_execution_src: String,
    /// Span within the source text of the input type.
    #[cfg(feature = "error_reporting")]
    #[label("This type is not present in `resources`")]
    pub input_span: Option<miette::SourceSpan>,
    /// Full span so that miette renders the whole `cmd_execution_src`.
    #[cfg(feature = "error_reporting")]
    #[label]
    pub full_span: miette::SourceSpan,
}

#[cfg(feature = "error_reporting")]
impl<'b> Borrow<dyn miette::Diagnostic + 'b> for Box<InputFetchError> {
    fn borrow<'s>(&'s self) -> &'s (dyn miette::Diagnostic + 'b) {
        self.as_ref()
    }
}

#[cfg(feature = "error_reporting")]
impl miette::Diagnostic for Box<InputFetchError> {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.as_ref().code()
    }

    fn severity(&self) -> Option<miette::Severity> {
        self.as_ref().severity()
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.as_ref().help()
    }

    fn url<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.as_ref().url()
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        self.as_ref().source_code()
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        self.as_ref().labels()
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn miette::Diagnostic> + 'a>> {
        self.as_ref().related()
    }

    fn diagnostic_source(&self) -> Option<&dyn miette::Diagnostic> {
        self.as_ref().diagnostic_source()
    }
}
