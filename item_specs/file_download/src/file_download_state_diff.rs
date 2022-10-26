use std::{fmt, path::PathBuf};

use peace::diff::{Changeable, Tracked};
use serde::{Deserialize, Serialize};

/// Diff between the current and desired downloaded file.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum FileDownloadStateDiff {
    /// File exists locally, but does not exist on server.
    Deleted {
        /// Path to the file.
        path: PathBuf,
    },
    /// File does not exist both locally and on server.
    NoChangeNonExistent {
        /// Path to the file.
        path: PathBuf,
    },
    /// File exists both locally and on server, and they are in sync.
    NoChangeSync {
        /// Path to the file.
        path: PathBuf,
    },
    /// There is a change.
    Change {
        /// Path to the file.
        path: PathBuf,
        /// Possible change in byte length.
        byte_len: Changeable<usize>,
        /// Possible change in contents.
        contents: Changeable<String>,
    },
}

impl fmt::Display for FileDownloadStateDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deleted { path } => write!(
                f,
                "resource does not exist on server; locally `{}` exists, but ensure will not delete it",
                path.display()
            ),
            Self::NoChangeNonExistent { path } => write!(
                f,
                "resource does not exist on server, and `{}` does not exist locally",
                path.display()
            ),
            Self::NoChangeSync { path } => write!(f, "`{}` in sync with server", path.display()),
            Self::Change {
                path,
                byte_len,
                contents: _,
            } => {
                write!(f, "`{}` will change", path.display())?;

                match byte_len.from {
                    Tracked::None => {
                        write!(f, " from 0 bytes")?;
                    }
                    Tracked::Unknown => {}
                    Tracked::Known(byte_count) => {
                        write!(f, " from {byte_count} bytes")?;
                    }
                }

                match byte_len.to {
                    Tracked::None => {
                        write!(f, " to 0 bytes")?;
                    }
                    Tracked::Unknown => {
                        write!(f, " to unknown size")?;
                    }
                    Tracked::Known(byte_count) => {
                        write!(f, " to {byte_count} bytes")?;
                    }
                }

                Ok(())
            }
        }
    }
}
