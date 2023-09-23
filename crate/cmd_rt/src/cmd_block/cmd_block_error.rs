use std::fmt::Debug;

use peace_resources::ResourceFetchError;
use peace_rt_model::outcomes::CmdOutcome;

/// Error while executing a `CmdBlock`.
///
/// # Type Parameters
///
/// * `T`: Execution outcome, mapped from `CmdBlock::OutcomeAcc`.
/// * `E`: Application error type.
#[derive(Debug, thiserror::Error)]
pub enum CmdBlockError<T, E>
where
    T: Debug,
    E: Debug,
{
    /// Error fetching `CmdBlock::InputT` from `resources`.
    ///
    /// If `CmdBlock::InputT` is a tuple, such as `(StatesCurrent, StatesGoal)`,
    /// and `states_current` and `states_goal` are inserted individually in
    /// `Resources`, then `CmdBlock::input_fetch` should be implemented to call
    /// `Resources::remove` for each of them.
    #[error(
        "Failed to fetch `{input_name_short}` from `resource`s.",
        input_name_short = _0.resource_name_short
    )]
    InputFetch(
        #[source]
        #[from]
        ResourceFetchError,
    ),
    /// Error originated from `CmdBlock` exec/collate code.
    #[error("`CmdBlock` block execution or collation logic failed.")]
    Block(E),
    /// Error originated from at least one item.
    ///
    /// The `CmdBlock::OutcomeAcc` is not returned in this variant, but
    /// is mapped to the `ExecutionOutcome`.
    #[error("`CmdBlock` item logic failed.")]
    Outcome(CmdOutcome<T, E>),
}
