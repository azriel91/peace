use std::path::{Path, PathBuf};

use peace::data::{Data, R};
use url::Url;

/// Download parameters from the user.
#[derive(Data, Debug)]
pub struct DownloadParams<'op> {
    /// Url of the file to download.
    src: R<'op, Option<Url>>,
    /// Path of the destination.
    ///
    /// Must be a file path, and not a directory.
    dest: R<'op, Option<PathBuf>>,
}

impl<'op> DownloadParams<'op> {
    pub fn new(src: R<'op, Option<Url>>, dest: R<'op, Option<PathBuf>>) -> Self {
        Self { src, dest }
    }

    pub fn src(&self) -> Option<&Url> {
        self.src.as_ref()
    }

    pub fn dest(&self) -> Option<&Path> {
        self.dest.as_deref()
    }
}
