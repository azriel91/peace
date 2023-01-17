#[cfg(target_arch = "wasm32")]
use peace::rt_model::Storage;

use peace::{
    cfg::{state::FetchedOpt, Saved, State},
    data::{Data, R},
};

use crate::{ETag, FileDownloadParams, FileDownloadState};

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

    /// The previous file download state.
    state_prev: Saved<'op, State<FileDownloadState, FetchedOpt<ETag>>>,

    /// For wasm, we write to web storage through the `Storage` object.
    ///
    /// If `rt_model_native::Storage` exposed similar API, then storage
    /// operations for item spec implementations will be easier.
    #[cfg(target_arch = "wasm32")]
    storage: R<'op, Storage>,
}

impl<'op, Id> FileDownloadData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn file_download_params(&self) -> &FileDownloadParams<Id> {
        &self.file_download_params
    }

    pub fn state_prev(&self) -> &Saved<'op, State<FileDownloadState, FetchedOpt<ETag>>> {
        &self.state_prev
    }

    #[cfg(target_arch = "wasm32")]
    pub fn storage(&self) -> &Storage {
        &*self.storage
    }
}
