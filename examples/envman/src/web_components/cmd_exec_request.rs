use serde::{Deserialize, Serialize};

/// Request for a command execution.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CmdExecRequest {
    /// Run the `StatesDiscoverCmd`.
    Discover,
    /// Run the `EnsureCmd`.
    Ensure,
    /// Run the `CleanCmd`.
    Clean,
}
