use serde::{Deserialize, Serialize};

/// When resolving `Value`s, whether to look up `Current<T>` or `Goal<T>`.
///
/// # Design
///
/// Remember to update these places when updating here.
///
/// 1. Marker types in `crate/data/src/marker.rs`.
/// 2. `peace_params::MappingFnImpl`.
/// 3. Resource insertions in `ItemWrapper::setup`.
//
// TODO: Should we have modes for:
//
// * `CurrentStored`
// * `GoalStored`
// * `ExecutionBeginning`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueResolutionMode {
    /// Resolve values using example states.
    #[cfg(feature = "item_state_example")]
    Example,
    /// Resolve values using cleaned states.
    Clean,
    /// Resolve values using current states.
    Current,
    /// Resolve values using goal states.
    Goal,
    /// Resolve values using dry-applied states.
    ///
    /// The states in memory may be example / fake / placeholder values.
    ///
    /// TODO: resolve this in [#196]
    ///
    /// [#196]: https://github.com/azriel91/peace/issues/196
    ApplyDry,
}
