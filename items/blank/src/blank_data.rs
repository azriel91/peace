use peace::data::{accessors::W, Data};

use crate::BlankParams;

/// Data used to manage blank state.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different blank parameters
///   from each other.
#[derive(Data, Debug)]
pub struct BlankData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    /// Blank state parameters.
    params: W<'exec, BlankParams<Id>>,
}

impl<'exec, Id> BlankData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    pub fn new(params: W<'exec, BlankParams<Id>>) -> Self {
        Self { params }
    }

    pub fn params(&self) -> &BlankParams<Id> {
        &self.params
    }

    pub fn params_mut(&mut self) -> &mut BlankParams<Id> {
        &mut self.params
    }
}
