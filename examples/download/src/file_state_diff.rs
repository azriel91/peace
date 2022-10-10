use std::fmt;

use peace::diff::{Changeable, Tracked};
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

impl fmt::Display for FileStateDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deleted => write!(f, "server file deleted"),
            Self::NoChangeNonExistent => write!(f, "no file to download"),
            Self::NoChangeSync => write!(f, "file in sync with server"),
            Self::Change {
                byte_len,
                contents: _,
            } => {
                write!(f, "file will change")?;

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
