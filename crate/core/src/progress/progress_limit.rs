use serde::{Deserialize, Serialize};

/// Unit of measurement and total number of units.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ProgressLimit {
    /// There is no meaningful way to measure progress.
    Unknown,
    /// Progress is complete when `n` steps have been completed.
    Steps(u64),
    /// Progress is complete when `n` bytes are processed.
    ///
    /// Useful for upload / download progress.
    Bytes(u64),
}

impl Default for ProgressLimit {
    fn default() -> Self {
        Self::Unknown
    }
}
