use std::marker::PhantomData;

use peace::cfg::{async_trait, state::Nothing, CleanOpSpec, OpCheckStatus, ProgressLimit, State};

use crate::{BlankData, BlankError, BlankState};

/// `CleanOpSpec` for the blank state.
#[derive(Debug, Default)]
pub struct BlankCleanOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> CleanOpSpec for BlankCleanOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = BlankData<'op, Id>;
    type Error = BlankError;
    type StateLogical = BlankState;
    type StatePhysical = Nothing;

    async fn check(
        _blank_data: BlankData<'_, Id>,
        state_current: &State<BlankState, Nothing>,
    ) -> Result<OpCheckStatus, BlankError> {
        let blank_state = &state_current.logical;
        let op_check_status = if blank_state.is_none() {
            OpCheckStatus::ExecNotRequired
        } else {
            let progress_limit = ProgressLimit::Steps(1);
            OpCheckStatus::ExecRequired { progress_limit }
        };

        Ok(op_check_status)
    }

    async fn exec_dry(
        _blank_data: BlankData<'_, Id>,
        _state_current: &State<BlankState, Nothing>,
    ) -> Result<(), BlankError> {
        Ok(())
    }

    async fn exec(
        mut blank_data: BlankData<'_, Id>,
        _state_current: &State<BlankState, Nothing>,
    ) -> Result<(), BlankError> {
        let dest = blank_data.params_mut().dest_mut();
        **dest = None;

        Ok(())
    }
}
