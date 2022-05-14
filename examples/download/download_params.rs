use std::path::{Path, PathBuf};

use url::Url;

/// Download parameters from the user.
#[derive(Debug)]
pub struct DownloadParams {
    /// Url of the file to download.
    src: Url,
    /// Path of the destination.
    ///
    /// Must be a file path, and not a directory.
    dest: PathBuf,
}

impl DownloadParams {
    pub fn new(src: Url, dest: PathBuf) -> Self {
        Self { src, dest }
    }

    pub fn src(&self) -> &Url {
        &self.src
    }

    pub fn dest(&self) -> &Path {
        &self.dest
    }
}
