use crate::states::{ts::Clean, States};

/// Clean `State`s for all `Item`s.
///
/// These are the states that each item would be in, if `Item::apply` were to be
/// run with `state_clean` as the target state.
///
/// **Note:** Not to be confused with [`StatesCleaned`].
///
/// [`StatesCleaned`]: crate::states::StatesCleaned
///
/// # Implementors
///
/// You may reference [`StatesClean`] after `CleanCmd::exec` has been run,
/// unless it is the `ExecutionOutcome`.
pub type StatesClean = States<Clean>;
