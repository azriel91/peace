use peace::data::{Data, W};

use crate::BlankParams;

/// Data used to manage blank state.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different blank parameters
///   from each other.
#[derive(Data, Debug)]
pub struct BlankData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    /// Blank state parameters.
    params: W<'op, BlankParams<Id>>,
}

impl<'op, Id> BlankData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    pub fn new(params: W<'op, BlankParams<Id>>) -> Self {
        Self { params }
    }

    pub fn params(&self) -> &BlankParams<Id> {
        &self.params
    }

    pub fn params_mut(&mut self) -> &mut BlankParams<Id> {
        &mut self.params
    }
}
