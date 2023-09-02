use std::fmt::Debug;

use peace_rt_model::outcomes::CmdOutcome;

/// Error while executing a `CmdBlock`.
///
/// # Type Parameters
///
/// * `T`: Execution outcome, mapped from `CmdBlock::OutcomeAcc`.
/// * `E`: Application error type.
#[cfg_attr(feature = "error_reporting", derive(miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum CmdBlockError<T, E>
where
    T: Debug,
    E: Debug,
{
    /// Error originated from `CmdBlock` code.
    #[error("`CmdBlock` block logic failed.")]
    Block(E),
    /// Error originated from at least one item.
    ///
    /// The `CmdBlock::OutcomeAcc` is not returned in this variant, but
    /// is mapped to the `ExecutionOutcome`.
    #[error("`CmdBlock` item logic failed.")]
    Outcome(CmdOutcome<T, E>),
}
