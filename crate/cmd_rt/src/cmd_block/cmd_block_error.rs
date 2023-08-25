use std::fmt::Debug;

use peace_cfg::ItemId;
use peace_rt_model::IndexMap;

/// Error while executing a `CmdBlock`.
#[cfg_attr(feature = "error_reporting", derive(miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum CmdBlockError<OutcomeAcc, E>
where
    OutcomeAcc: Debug,
    E: Debug,
{
    /// Error originated from `CmdBlock` code.
    #[error("`CmdBlock` block logic failed.")]
    Block {
        /// Outcome accumulator at the point of error.
        outcome_acc: OutcomeAcc,
        /// Error that occurred.
        error: E,
    },
    /// Error originated from at least one item.
    #[error("`CmdBlock` item logic failed.")]
    Item {
        /// Outcome accumulator at the point of error.
        outcome_acc: OutcomeAcc,
        /// Error(s) from the item executions.
        error: IndexMap<ItemId, E>,
    },
}
