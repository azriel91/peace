use std::path::{Path, PathBuf};

use peace::data::{Data, R};
use url::Url;

/// Download parameters from the user.
#[derive(Data, Debug)]
pub struct DownloadParams<'op> {
    /// Client to make web requests.
    client: R<'op, reqwest::Client>,
    /// Url of the file to download.
    src: R<'op, Url>,
    /// Path of the destination.
    ///
    /// Must be a file path, and not a directory.
    dest: R<'op, PathBuf>,
}

impl<'op> DownloadParams<'op> {
    pub fn new(client: R<'op, reqwest::Client>, src: R<'op, Url>, dest: R<'op, PathBuf>) -> Self {
        Self { client, src, dest }
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn src(&self) -> &Url {
        &self.src
    }

    pub fn dest(&self) -> &Path {
        self.dest.as_ref()
    }
}
