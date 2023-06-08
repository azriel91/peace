use std::{fmt, path::PathBuf};

use serde::{Deserialize, Serialize};

/// State of the contents of the file to download.
///
/// This is used to represent the state of the source file, as well as the
/// destination file.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FileDownloadState {
    /// File does not exist.
    None {
        /// Path to the tracked file, if any.
        path: Option<PathBuf>,
    },
    /// String contents of the file.
    ///
    /// Use this when:
    ///
    /// * File contents is text.
    /// * File is small enough to load in memory.
    StringContents {
        /// Path to the file.
        path: PathBuf,
        /// Contents of the file.
        contents: String,
    },
    /// Length of the file.
    ///
    /// Use this when:
    ///
    /// * File is not practical to load in memory.
    Length {
        /// Path to the file.
        path: PathBuf,
        /// Number of bytes.
        byte_count: u64,
    },
    /// Cannot determine file state.
    ///
    /// May be used for the goal state
    Unknown {
        /// Path to the file.
        path: PathBuf,
    },
}

impl fmt::Display for FileDownloadState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None { path } => {
                if let Some(path) = path {
                    let path = path.display();
                    write!(f, "`{path}` non-existent")
                } else {
                    write!(f, "non-existent")
                }
            }
            Self::StringContents { path, contents } => {
                let path = path.display();
                write!(f, "`{path}` containing \"{contents}\"")
            }
            Self::Length { path, byte_count } => {
                let path = path.display();
                write!(f, "`{path}` containing {byte_count} bytes")
            }
            Self::Unknown { path } => {
                let path = path.display();
                write!(f, "`{path}` (contents not tracked)")
            }
        }
    }
}
