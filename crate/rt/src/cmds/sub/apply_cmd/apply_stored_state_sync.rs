/// Whether to block an apply operation if stored states are not in sync with
/// discovered state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApplyStoredStateSync {
    /// Neither stored current states nor stored goal state need to be in sync
    /// with the discovered current states and goal state.
    None,
    /// The stored current states must be in sync with the discovered current
    /// state for the apply to proceed.
    ///
    /// The stored goal state does not need to be in sync with the discovered
    /// goal state.
    Current,
    /// The stored goal state must be in sync with the discovered goal
    /// state for the apply to proceed.
    ///
    /// The stored current states does not need to be in sync with the
    /// discovered current state.
    ///
    /// For `CleanCmd`, this variant is equivalent to `None`.
    Goal,
    /// Both stored current states and stored goal state must be in sync with
    /// the discovered current states and goal state for the apply to
    /// proceed.
    ///
    /// For `CleanCmd`, this variant is equivalent to `Current`.
    Both,
}
