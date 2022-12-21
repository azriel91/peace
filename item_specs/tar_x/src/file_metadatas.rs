use std::fmt;

use serde::{Deserialize, Serialize};

use crate::FileMetadata;

/// Metadata of files to extract.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileMetadatas(Vec<FileMetadata>);

impl FileMetadatas {
    /// Returns a new `FileMetadatas`.
    pub fn new(file_metadatas: Vec<FileMetadata>) -> FileMetadatas {
        Self(file_metadatas)
    }

    /// Returns the inner `Vec<FileMetadata>`.
    pub fn into_inner(self) -> Vec<FileMetadata> {
        self.0
    }
}

impl std::ops::Deref for FileMetadatas {
    type Target = Vec<FileMetadata>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for FileMetadatas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for FileMetadatas {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
