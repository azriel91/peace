use std::{
    fmt,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use url::Url;

/// File download parameters.
///
/// The `Id` type parameter is needed for each file download params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different file download
///   parameters from each other.
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct FileDownloadParams<Id> {
    /// Url of the file to download.
    src: Url,
    /// Path of the destination.
    ///
    /// Must be a file path, and not a directory.
    dest: PathBuf,
    /// Marker for unique download parameters type.
    marker: PhantomData<Id>,
}

impl<Id> FileDownloadParams<Id> {
    /// Returns a new `DownloadProfileInit`.
    pub fn new(src: Url, dest: PathBuf) -> Self {
        Self {
            src,
            dest,
            marker: PhantomData,
        }
    }

    /// Returns the URL to download from.
    pub fn src(&self) -> &Url {
        &self.src
    }

    /// Returns the file path to write to.
    pub fn dest(&self) -> &Path {
        &self.dest
    }
}

impl<Id> fmt::Debug for FileDownloadParams<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileDownloadParams")
            .field("src", &self.src)
            .field("dest", &self.dest)
            .finish()
    }
}
