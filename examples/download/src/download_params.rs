#[cfg(target_arch = "wasm32")]
use std::path::PathBuf;

#[cfg(target_arch = "wasm32")]
use peace::data::W;

use peace::data::{Data, R};

use crate::DownloadProfileInit;

/// Download parameters from the user.
#[derive(Data, Debug)]
pub struct DownloadParams<'op> {
    /// Client to make web requests.
    client: R<'op, reqwest::Client>,
    /// Url of the file to download.
    download_profile_init: R<'op, DownloadProfileInit>,

    // For wasm, we use a map to hold the file content.
    #[cfg(target_arch = "wasm32")]
    in_memory_contents: W<'op, std::collections::HashMap<PathBuf, String>>,
}

impl<'op> DownloadParams<'op> {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(
        client: R<'op, reqwest::Client>,
        download_profile_init: R<'op, DownloadProfileInit>,
    ) -> Self {
        Self {
            client,
            download_profile_init,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(
        client: R<'op, reqwest::Client>,
        download_profile_init: R<'op, DownloadProfileInit>,
        in_memory_contents: W<'op, std::collections::HashMap<PathBuf, String>>,
    ) -> Self {
        Self {
            client,
            download_profile_init,
            in_memory_contents,
        }
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn download_profile_init(&self) -> &DownloadProfileInit {
        &self.download_profile_init
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
