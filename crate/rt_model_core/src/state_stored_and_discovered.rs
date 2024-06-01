use type_reg::untagged::BoxDtDisplay;

/// Stored and/or discovered state for a step.
#[derive(Clone, Debug)]
pub enum StateStoredAndDiscovered {
    /// Stored state exists, but the actual item state cannot be discovered.
    ///
    /// These can probably be ignored during `CleanCmd`, for idempotence even if
    /// a previous clean up did not complete successfully and stored states
    /// were not updated.
    OnlyStoredExists {
        /// Stored current state or stored goal state.
        state_stored: BoxDtDisplay,
    },
    /// No state was stored, but the actual item state exists.
    ///
    /// These can probably be ignored during `EnsureCmd`, for idempotence even
    /// if a previous ensure did not complete successfully and stored states
    /// were not updated.
    OnlyDiscoveredExists {
        /// Discovered current state or stored goal state during execution.
        state_discovered: BoxDtDisplay,
    },
    /// Both stored state and discovered state exist.
    ///
    /// This variant is the one that users likely should be warned when ensuring
    /// changes.
    ValuesDiffer {
        /// Stored current state or stored goal state.
        state_stored: BoxDtDisplay,
        /// Discovered current state or stored goal state during execution.
        state_discovered: BoxDtDisplay,
    },
}
