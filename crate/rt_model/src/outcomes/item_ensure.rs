use std::fmt::{Debug, Display};

use peace_cfg::OpCheckStatus;
use peace_resources::type_reg::untagged::{DataType, DataTypeDisplay};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::outcomes::{ItemEnsurePartial, ItemEnsureRt};

/// Information about an item during an `EnsureCmd` execution.
///
/// This is similar to [`ItemEnsurePartial`], with most fields being
/// non-optional, and the added `state_ensured` field.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ItemEnsure<State, StateDiff> {
    /// State saved on disk before the execution.
    pub state_saved: Option<State>,
    /// Current state discovered during the execution.
    pub state_current: State,
    /// Desired state discovered during the execution.
    pub state_desired: State,
    /// Diff between current and desired states.
    pub state_diff: StateDiff,
    /// Whether item execution was required.
    pub op_check_status: OpCheckStatus,
    /// The state that was ensured, `None` if execution was not required.
    pub state_ensured: Option<State>,
}

impl<State, StateDiff> TryFrom<(ItemEnsurePartial<State, StateDiff>, Option<State>)>
    for ItemEnsure<State, StateDiff>
{
    type Error = (ItemEnsurePartial<State, StateDiff>, Option<State>);

    fn try_from(
        (partial, state_ensured): (ItemEnsurePartial<State, StateDiff>, Option<State>),
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

impl<State, StateDiff> ItemEnsureRt for ItemEnsure<State, StateDiff>
where
    State: Clone + Debug + Display + Serialize + DeserializeOwned + Send + Sync + 'static,
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
