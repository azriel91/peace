use std::fmt;

use serde::{Deserialize, Serialize};

use crate::FileMetadata;

/// State of the tar extraction.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TarXState {
    /// Files in the tar are in sync with extraction location.
    ExtractionInSync {
        /// Metadata of files in the tar.
        files_in_tar: Vec<FileMetadata>,
    },
    /// Files in the tar are not sync with extraction location.
    ExtractionOutOfSync {
        /// Metadata of files in the tar.
        files_in_tar: Vec<FileMetadata>,
        /// Metadata of files existent at the extraction location.
        files_extracted: Vec<FileMetadata>,
    },
}

impl fmt::Display for TarXState {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}