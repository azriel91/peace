use std::marker::PhantomData;

use peace::{
    cfg::{accessors::Stored, State},
    data::Data,
};

use crate::{ShCmdExecutionRecord, ShCmdState};

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
    /// Stored states of this item's previous execution.
    state_current_stored: Stored<'exec, State<ShCmdState<Id>, ShCmdExecutionRecord>>,

    /// Marker.
    marker: PhantomData<Id>,
}

impl<'exec, Id> ShCmdData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    /// Returns the previous states.
    pub fn state_current_stored(&self) -> Option<&State<ShCmdState<Id>, ShCmdExecutionRecord>> {
        self.state_current_stored.get()
    }
}
