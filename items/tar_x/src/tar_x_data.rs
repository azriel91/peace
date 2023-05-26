use std::marker::PhantomData;

use peace::{
    data::{accessors::R, Data},
    rt_model::Storage,
};

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
    /// Storage to interact with to read the tar file / extract to.
    storage: R<'exec, Storage>,

    /// Marker.
    marker: PhantomData<Id>,
}

impl<'exec, Id> TarXData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    pub fn storage(&self) -> &Storage {
        &self.storage
    }
}
