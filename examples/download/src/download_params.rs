use std::path::{Path, PathBuf};

#[cfg(target_arch = "wasm32")]
use peace::data::W;

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

    // For wasm, we use a map to hold the file content.
    #[cfg(target_arch = "wasm32")]
    in_memory_contents: W<'op, std::collections::HashMap<PathBuf, String>>,
}

impl<'op> DownloadParams<'op> {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(client: R<'op, reqwest::Client>, src: R<'op, Url>, dest: R<'op, PathBuf>) -> Self {
        Self { client, src, dest }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(
        client: R<'op, reqwest::Client>,
        src: R<'op, Url>,
        dest: R<'op, PathBuf>,
        in_memory_contents: W<'op, std::collections::HashMap<PathBuf, String>>,
    ) -> Self {
        Self {
            client,
            src,
            dest,
            in_memory_contents,
        }
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

    #[cfg(target_arch = "wasm32")]
    pub fn in_memory_contents(&self) -> &std::collections::HashMap<PathBuf, String> {
        &*self.in_memory_contents
    }

    #[cfg(target_arch = "wasm32")]
    pub fn in_memory_contents_mut(&mut self) -> &mut std::collections::HashMap<PathBuf, String> {
        &mut *self.in_memory_contents
    }
}
