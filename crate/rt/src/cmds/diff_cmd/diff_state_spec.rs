/// Whether to read stored state or discover state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiffStateSpec {
    /// Discovers the current state upon execution.
    Current,
    /// Reads previously stored current state.
    CurrentStored,
    /// Discovers the goal state upon execution.
    Goal,
    /// Reads previously stored goal state.
    GoalStored,
}
