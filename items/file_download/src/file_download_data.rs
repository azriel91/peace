use std::marker::PhantomData;

#[cfg(target_arch = "wasm32")]
use peace::rt_model::Storage;

use peace::{
    cfg::accessors::Stored,
    data::{accessors::R, marker::Current, Data},
};

use crate::FileDownloadState;

/// Data used to download a file.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different file download
///   parameters from each other.
#[derive(Data, Debug)]
pub struct FileDownloadData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    /// Client to make web requests.
    client: R<'exec, reqwest::Client>,

    /// The previous file download state.
    state_prev: Stored<'exec, FileDownloadState>,

    /// The file state working copy in memory.
    state_working: R<'exec, Current<FileDownloadState>>,

    /// For wasm, we write to web storage through the `Storage` object.
    ///
    /// If `rt_model_native::Storage` exposed similar API, then storage
    /// operations for item implementations will be easier.
    #[cfg(target_arch = "wasm32")]
    storage: R<'exec, Storage>,

    /// Marker.
    marker: PhantomData<Id>,
}

impl<'exec, Id> FileDownloadData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn state_prev(&self) -> &Stored<'exec, FileDownloadState> {
        &self.state_prev
    }

    pub fn state_working(&self) -> &Current<FileDownloadState> {
        &self.state_working
    }

    #[cfg(target_arch = "wasm32")]
    pub fn storage(&self) -> &Storage {
        &self.storage
    }
}
