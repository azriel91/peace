use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, CleanOpSpec, OpCheckStatus};

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
    type State = BlankState;

    async fn check(
        _blank_data: BlankData<'_, Id>,
        blank_state: &BlankState,
    ) -> Result<OpCheckStatus, BlankError> {
        let op_check_status = if blank_state.is_none() {
            OpCheckStatus::ExecNotRequired
        } else {
            #[cfg(not(feature = "output_progress"))]
            {
                OpCheckStatus::ExecRequired
            }
            #[cfg(feature = "output_progress")]
            {
                let progress_limit = ProgressLimit::Steps(1);
                OpCheckStatus::ExecRequired { progress_limit }
            }
        };

        Ok(op_check_status)
    }

    async fn exec_dry(
        _blank_data: BlankData<'_, Id>,
        _state_current: &BlankState,
    ) -> Result<(), BlankError> {
        Ok(())
    }

    async fn exec(
        mut blank_data: BlankData<'_, Id>,
        _state_current: &BlankState,
    ) -> Result<(), BlankError> {
        let dest = blank_data.params_mut().dest_mut();
        **dest = None;

        Ok(())
    }
}
