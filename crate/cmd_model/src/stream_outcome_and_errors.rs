use fn_graph::StreamOutcome;
use indexmap::IndexMap;
use peace_cfg::StepId;

/// `CmdBlock` stream outcome and step wise errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StreamOutcomeAndErrors<T, E> {
    /// The `CmdBlock` stream outcome.
    pub stream_outcome: StreamOutcome<T>,
    /// The errors during processing,
    pub errors: IndexMap<StepId, E>,
}
