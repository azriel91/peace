use std::{fmt, path::PathBuf};

use serde::{Deserialize, Serialize};

#[cfg(feature = "output_progress")]
use peace::item_model::ItemLocationState;

/// State of the contents of the file to download.
///
/// This is used to represent the state of the source file, as well as the
/// destination file.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[cfg(feature = "output_progress")]
impl<'state> From<&'state FileDownloadState> for ItemLocationState {
    fn from(file_download_state: &'state FileDownloadState) -> ItemLocationState {
        match file_download_state {
            FileDownloadState::None { .. } => ItemLocationState::NotExists,
            FileDownloadState::StringContents { .. }
            | FileDownloadState::Length { .. }
            | FileDownloadState::Unknown { .. } => todo!(),
        }
    }
}

impl PartialEq for FileDownloadState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                FileDownloadState::None { path: path_self },
                FileDownloadState::None { path: path_other },
            ) => path_self == path_other,
            (
                FileDownloadState::Unknown {
                    path: path_self, ..
                },
                FileDownloadState::StringContents {
                    path: path_other, ..
                },
            )
            | (
                FileDownloadState::Unknown {
                    path: path_self, ..
                },
                FileDownloadState::Length {
                    path: path_other, ..
                },
            )
            | (
                FileDownloadState::StringContents {
                    path: path_self, ..
                },
                FileDownloadState::Unknown {
                    path: path_other, ..
                },
            )
            | (
                FileDownloadState::Length {
                    path: path_self, ..
                },
                FileDownloadState::Unknown {
                    path: path_other, ..
                },
            )
            | (
                FileDownloadState::Unknown { path: path_self },
                FileDownloadState::Unknown { path: path_other },
            ) => path_self == path_other,

            (FileDownloadState::Unknown { .. }, FileDownloadState::None { .. })
            | (FileDownloadState::None { .. }, FileDownloadState::Unknown { .. })
            | (FileDownloadState::None { .. }, FileDownloadState::StringContents { .. })
            | (FileDownloadState::StringContents { .. }, FileDownloadState::None { .. })
            | (FileDownloadState::StringContents { .. }, FileDownloadState::Length { .. })
            | (FileDownloadState::Length { .. }, FileDownloadState::None { .. })
            | (FileDownloadState::Length { .. }, FileDownloadState::StringContents { .. })
            | (FileDownloadState::None { .. }, FileDownloadState::Length { .. }) => false,
            (
                FileDownloadState::StringContents {
                    path: path_self,
                    contents: contents_self,
                },
                FileDownloadState::StringContents {
                    path: path_other,
                    contents: contents_other,
                },
            ) => path_self == path_other && contents_self == contents_other,
            (
                FileDownloadState::Length {
                    path: path_self,
                    byte_count: byte_count_self,
                },
                FileDownloadState::Length {
                    path: path_other,
                    byte_count: byte_count_other,
                },
            ) => path_self == path_other && byte_count_self == byte_count_other,
        }
    }
}
