use peace::diff::Changeable;
use serde::{Deserialize, Serialize};

/// Diff between the current and desired downloaded file.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum FileStateDiff {
    /// File exists locally, but does not exist on server.
    Deleted,
    /// File does not exist both locally and on server.
    NoChangeNonExistent,
    /// File exists both locally and on server, and they are in sync.
    NoChangeSync,
    /// There is a change.
    Change {
        /// Possible change in byte length.
        byte_len: Changeable<usize>,
        /// Possible change in contents.
        contents: Changeable<String>,
    },
}
