use peace::{
    data::{Data, R},
    rt_model::Storage,
};

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

    /// Storage to interact with to read the tar file / extract to.
    storage: R<'op, Storage>,
}

impl<'op, Id> TarXData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    pub fn new(tar_x_params: R<'op, TarXParams<Id>>, storage: R<'op, Storage>) -> Self {
        Self {
            tar_x_params,
            storage,
        }
    }

    pub fn tar_x_params(&self) -> &TarXParams<Id> {
        &self.tar_x_params
    }

    pub fn storage(&self) -> &Storage {
        &*self.storage
    }
}
