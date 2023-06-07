use serde::{Deserialize, Serialize};

/// When resolving `Value`s, whether to look up `Current<T>` or `Goal<T>`.
//
// Corresponds to marker types in `crate/data/src/marker.rs`.
// Remember to update there when updating here.
//
// TODO: Should we have modes for:
//
// * `CurrentStored`
// * `GoalStored`
// * `ExecutionBeginning`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueResolutionMode {
    /// Resolve values using dry-applied states.
    ///
    /// The states in memory may be example / fake / placeholder values.
    ApplyDry,
    /// Resolve values using current states.
    Current,
    /// Resolve values using goal states.
    Goal,
    /// Resolve values using cleaned states.
    Clean,
}
