#[cfg(target_arch = "wasm32")]
use peace::rt_model::Storage;

use peace::data::{Data, R};

use crate::DownloadProfileInit;

/// Download parameters from the user.
#[derive(Data, Debug)]
pub struct DownloadParams<'op> {
    /// Client to make web requests.
    client: R<'op, reqwest::Client>,
    /// Url of the file to download.
    download_profile_init: R<'op, DownloadProfileInit>,

    /// For wasm, we write to web storage through the `Storage` object.
    ///
    /// Presumably we should be able to use this for `NativeStorage` as well.
    #[cfg(target_arch = "wasm32")]
    storage: R<'op, Storage>,
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
        storage: R<'op, Storage>,
    ) -> Self {
        Self {
            client,
            download_profile_init,
            storage,
        }
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn download_profile_init(&self) -> &DownloadProfileInit {
        &self.download_profile_init
    }

    #[cfg(target_arch = "wasm32")]
    pub fn storage(&self) -> &Storage {
        &*self.storage
    }
}
