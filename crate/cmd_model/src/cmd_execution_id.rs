use std::ops::Deref;

/// ID of a command execution.
///
/// Uniqueness is not yet defined -- these may overlap with IDs from different
/// machines.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CmdExecutionId(u64);

impl CmdExecutionId {
    /// Returns a new `CmdExecutionId`.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the underlying ID.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

impl Deref for CmdExecutionId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
