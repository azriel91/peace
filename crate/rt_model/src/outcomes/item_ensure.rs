use std::fmt::{Debug, Display};

use peace_cfg::{state::Placeholder, OpCheckStatus, State};
use peace_resources::type_reg::untagged::{DataType, DataTypeDisplay};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::outcomes::{ItemEnsurePartial, ItemEnsureRt};

/// Information about an item during an `EnsureCmd` execution.
///
/// This is similar to [`ItemEnsurePartial`], with most fields being
/// non-optional, and the added `state_ensured` field.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ItemEnsure<StateLogical, StatePhysical, StateDiff> {
    /// State saved on disk before the execution.
    pub state_saved: Option<State<StateLogical, StatePhysical>>,
    /// Current state discovered during the execution.
    pub state_current: State<StateLogical, StatePhysical>,
    /// Desired state discovered during the execution.
    pub state_desired: State<StateLogical, Placeholder>,
    /// Diff between current and desired states.
    pub state_diff: StateDiff,
    /// Whether item execution was required.
    pub op_check_status: OpCheckStatus,
    /// The state that was ensured, `None` if execution was not required.
    pub state_ensured: Option<State<StateLogical, StatePhysical>>,
}

impl<StateLogical, StatePhysical, StateDiff>
    TryFrom<(
        ItemEnsurePartial<StateLogical, StatePhysical, StateDiff>,
        Option<State<StateLogical, StatePhysical>>,
    )> for ItemEnsure<StateLogical, StatePhysical, StateDiff>
{
    type Error = (
        ItemEnsurePartial<StateLogical, StatePhysical, StateDiff>,
        Option<State<StateLogical, StatePhysical>>,
    );

    fn try_from(
        (partial, state_ensured): (
            ItemEnsurePartial<StateLogical, StatePhysical, StateDiff>,
            Option<State<StateLogical, StatePhysical>>,
        ),
    ) -> Result<Self, Self::Error> {
        let ItemEnsurePartial {
            state_saved,
            state_current,
            state_desired,
            state_diff,
            op_check_status,
        } = partial;

        if state_current.is_some()
            && state_desired.is_some()
            && state_diff.is_some()
            && op_check_status.is_some()
        {
            let (Some(state_current), Some(state_desired), Some(state_diff), Some(op_check_status)) =
                (state_current, state_desired, state_diff, op_check_status) else {
                    unreachable!("All are checked to be `Some` above.");
                };
            Ok(Self {
                state_saved,
                state_current,
                state_desired,
                state_diff,
                op_check_status,
                state_ensured,
            })
        } else {
            let partial = ItemEnsurePartial {
                state_saved,
                state_current,
                state_desired,
                state_diff,
                op_check_status,
            };
            Err((partial, state_ensured))
        }
    }
}

impl<StateLogical, StatePhysical, StateDiff> ItemEnsureRt
    for ItemEnsure<StateLogical, StatePhysical, StateDiff>
where
    StateLogical: Clone + Debug + Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Clone + Debug + Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateDiff: Clone + Debug + Display + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn state_saved(&self) -> Option<Box<dyn DataTypeDisplay>> {
        self.state_saved
            .clone()
            .map(|state_saved| Box::new(state_saved) as Box<dyn DataTypeDisplay>)
    }

    fn state_current(&self) -> Box<dyn DataTypeDisplay> {
        Box::new(self.state_current.clone())
    }

    fn state_desired(&self) -> Box<dyn DataTypeDisplay> {
        Box::new(self.state_desired.clone())
    }

    fn state_diff(&self) -> Box<dyn DataTypeDisplay> {
        Box::new(self.state_diff.clone())
    }

    fn op_check_status(&self) -> OpCheckStatus {
        self.op_check_status
    }

    fn state_ensured(&self) -> Option<Box<dyn DataTypeDisplay>> {
        self.state_ensured
            .clone()
            .map(|state_ensured| Box::new(state_ensured) as Box<dyn DataTypeDisplay>)
    }

    fn as_data_type(&self) -> &dyn DataType {
        self
    }

    fn as_data_type_mut(&mut self) -> &mut dyn DataType {
        self
    }
}
