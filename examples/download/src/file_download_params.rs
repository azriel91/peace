#[cfg(target_arch = "wasm32")]
use peace::rt_model::Storage;

use peace::data::{Data, R};

use crate::FileDownloadProfileInit;

/// Download parameters from the user.
#[derive(Data, Debug)]
pub struct FileDownloadParams<'op> {
    /// Client to make web requests.
    client: R<'op, reqwest::Client>,
    /// Url of the file to download.
    file_download_profile_init: R<'op, FileDownloadProfileInit>,

    /// For wasm, we write to web storage through the `Storage` object.
    ///
    /// Presumably we should be able to use this for `NativeStorage` as well.
    #[cfg(target_arch = "wasm32")]
    storage: R<'op, Storage>,
}

impl<'op> FileDownloadParams<'op> {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(
        client: R<'op, reqwest::Client>,
        file_download_profile_init: R<'op, FileDownloadProfileInit>,
    ) -> Self {
        Self {
            client,
            file_download_profile_init,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(
        client: R<'op, reqwest::Client>,
        file_download_profile_init: R<'op, FileDownloadProfileInit>,
        storage: R<'op, Storage>,
    ) -> Self {
        Self {
            client,
            file_download_profile_init,
            storage,
        }
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn file_download_profile_init(&self) -> &FileDownloadProfileInit {
        &self.file_download_profile_init
    }

    #[cfg(target_arch = "wasm32")]
    pub fn storage(&self) -> &Storage {
        &*self.storage
    }
}
