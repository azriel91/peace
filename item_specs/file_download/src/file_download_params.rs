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
    /// How to store the content of the download -- text or base64 encoded.
    ///
    /// For URLs that return binary content, this must be `Base64` as browser
    /// storage can only store text.
    #[cfg(target_arch = "wasm32")]
    storage_form: crate::StorageForm,
    /// Marker for unique download parameters type.
    marker: PhantomData<Id>,
}

impl<Id> FileDownloadParams<Id> {
    /// Returns new `FileDownloadParams`.
    pub fn new(
        src: Url,
        dest: PathBuf,
        #[cfg(target_arch = "wasm32")] storage_form: crate::StorageForm,
    ) -> Self {
        Self {
            src,
            dest,
            #[cfg(target_arch = "wasm32")]
            storage_form,
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

    /// Returns the storage form for the response.
    ///
    /// This only applies to the WASM target.
    #[cfg(target_arch = "wasm32")]
    pub fn storage_form(&self) -> crate::StorageForm {
        self.storage_form
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
