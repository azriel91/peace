use peace::diff::Diff;
use serde::{Deserialize, Serialize};

/// State of the file to download.
///
/// This is used to represent the state of the source file, as well as the
/// destination file.
#[derive(Clone, Debug, Diff, Serialize, Deserialize, PartialEq)]
pub enum FileState {
    /// String contents of the file.
    ///
    /// Use this when:
    ///
    /// * File contents is text.
    /// * File is small enough to load in memory.
    StringContents(String),
    /// Length of the file.
    ///
    /// Use this when:
    ///
    /// * File is not practical to load in memory.
    Length(u64),
    /// Cannot determine file state.
    ///
    /// May be used for the desired state
    Unknown,
}
