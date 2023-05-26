use std::marker::PhantomData;

use peace::{
    data::{accessors::RMaybe, Data},
    resources::states::StatesSaved,
};

/// Data used to run a shell command.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different command execution
///   parameters from each other.
#[derive(Data, Debug)]
pub struct ShCmdData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    /// Saved states with this item's previous execution.
    states_saved: RMaybe<'exec, StatesSaved>,

    /// Marker.
    marker: PhantomData<Id>,
}

impl<'exec, Id> ShCmdData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    /// Returns the previous states.
    pub fn states_saved(&self) -> Option<&StatesSaved> {
        self.states_saved.as_deref()
    }
}
