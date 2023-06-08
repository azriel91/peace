use std::marker::PhantomData;

use peace::{
    data::{accessors::RMaybe, Data},
    resources::states::StatesCurrentStored,
};

/// Data used to run a shell command.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different command execution
///   parameters from each other.
#[derive(Data, Debug)]
pub struct ShSyncCmdData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    /// Stored states of this item's previous execution.
    states_current_stored: RMaybe<'exec, StatesCurrentStored>,
    /// Marker.
    marker: PhantomData<Id>,
}

impl<'exec, Id> ShSyncCmdData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    /// Returns the previous states.
    pub fn states_current_stored(&self) -> Option<&StatesCurrentStored> {
        self.states_current_stored.as_deref()
    }
}
