use peace::{
    data::{accessors::R, Data},
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
pub struct TarXData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    /// Tar extraction parameters.
    tar_x_params: R<'exec, TarXParams<Id>>,

    /// Storage to interact with to read the tar file / extract to.
    storage: R<'exec, Storage>,
}

impl<'exec, Id> TarXData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    pub fn new(tar_x_params: R<'exec, TarXParams<Id>>, storage: R<'exec, Storage>) -> Self {
        Self {
            tar_x_params,
            storage,
        }
    }

    pub fn tar_x_params(&self) -> &TarXParams<Id> {
        &self.tar_x_params
    }

    pub fn storage(&self) -> &Storage {
        &self.storage
    }
}
