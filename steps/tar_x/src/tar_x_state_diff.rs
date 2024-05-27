use std::fmt;

use serde::{Deserialize, Serialize};

use crate::FileMetadatas;

/// Diff between the tar and extraction directory.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum TarXStateDiff {
    /// Files in the tar are in sync with extraction directory.
    ExtractionInSync,
    /// Files in the tar are not in sync with extraction directory.
    ExtractionOutOfSync {
        /// Files that existx in the tar but not the extraction directory.
        added: FileMetadatas,
        /// Files that exist in both the tar and extraction directory, but
        /// differ.
        modified: FileMetadatas,
        /// Files that exist in the extraction directory, but not in the tar.
        removed: FileMetadatas,
    },
}

impl fmt::Display for TarXStateDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExtractionInSync => write!(f, "files extracted and up to date"),
            Self::ExtractionOutOfSync {
                added,
                modified,
                removed,
            } => {
                let added = added.len();
                let modified = modified.len();
                let removed = removed.len();
                write!(
                    f,
                    "extraction out of sync: {added} files added, {modified} modified, {removed} removed"
                )
            }
        }
    }
}
