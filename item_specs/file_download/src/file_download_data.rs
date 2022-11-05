#[cfg(target_arch = "wasm32")]
use peace::rt_model::Storage;

use peace::data::{Data, R};

use crate::FileDownloadParams;

/// Data used to download a file.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different file download
///   parameters from each other.
#[derive(Data, Debug)]
pub struct FileDownloadData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    /// Client to make web requests.
    client: R<'op, reqwest::Client>,
    /// Url of the file to download.
    file_download_params: R<'op, FileDownloadParams<Id>>,

    /// For wasm, we write to web storage through the `Storage` object.
    ///
    /// Presumably we should be able to use this for `NativeStorage` as well.
    #[cfg(target_arch = "wasm32")]
    storage: R<'op, Storage>,
}

impl<'op, Id> FileDownloadData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(
        client: R<'op, reqwest::Client>,
        file_download_params: R<'op, FileDownloadParams<Id>>,
    ) -> Self {
        Self {
            client,
            file_download_params,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(
        client: R<'op, reqwest::Client>,
        file_download_params: R<'op, FileDownloadParams<Id>>,
        storage: R<'op, Storage>,
    ) -> Self {
        Self {
            client,
            file_download_params,
            storage,
        }
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn file_download_params(&self) -> &FileDownloadParams<Id> {
        &self.file_download_params
    }

    #[cfg(target_arch = "wasm32")]
    pub fn storage(&self) -> &Storage {
        &*self.storage
    }
}
