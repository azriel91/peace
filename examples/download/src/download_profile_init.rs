use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use url::Url;

/// User parameters for a download profile.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DownloadProfileInit {
    /// Url of the file to download.
    src: Url,
    /// Path of the destination.
    ///
    /// Must be a file path, and not a directory.
    dest: PathBuf,
}

impl DownloadProfileInit {
    /// Returns a new `DownloadProfileInit`.
    pub fn new(src: Url, dest: PathBuf) -> Self {
        Self { src, dest }
    }

    /// Returns the URL to download from.
    pub fn src(&self) -> &Url {
        &self.src
    }

    /// Returns the file path to write to.
    pub fn dest(&self) -> &PathBuf {
        &self.dest
    }
}
