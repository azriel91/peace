#[cfg(target_arch = "wasm32")]
use peace::rt_model::Storage;

use peace::data::{Data, R};

use crate::TarXParams;

/// Data used to extract a tar file.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different tar extraction
///   parameters from each other.
#[derive(Data, Debug)]
pub struct TarXData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    /// Tar extraction parameters.
    tar_x_params: R<'op, TarXParams<Id>>,

    /// For wasm, we write to web storage through the `Storage` object.
    ///
    /// Presumably we should be able to use this for `NativeStorage` as well.
    #[cfg(target_arch = "wasm32")]
    storage: R<'op, Storage>,
}

impl<'op, Id> TarXData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(tar_x_params: R<'op, TarXParams<Id>>) -> Self {
        Self { tar_x_params }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(tar_x_params: R<'op, TarXParams<Id>>, storage: R<'op, Storage>) -> Self {
        Self {
            tar_x_params,
            storage,
        }
    }

    pub fn tar_x_params(&self) -> &TarXParams<Id> {
        &self.tar_x_params
    }

    #[cfg(target_arch = "wasm32")]
    pub fn storage(&self) -> &Storage {
        &*self.storage
    }
}
