use fn_graph::StreamOutcome;
use indexmap::IndexMap;
use peace_cfg::ItemId;

/// `CmdBlock` stream outcome and item wise errors.
#[derive(Debug)]
pub struct StreamOutcomeAndErrors<T, E> {
    /// The `CmdBlock` stream outcome.
    pub stream_outcome: StreamOutcome<T>,
    /// The errors during processing,
    pub errors: IndexMap<ItemId, E>,
}
