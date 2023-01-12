use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, state::Nothing, EnsureOpSpec, OpCheckStatus, OpCtx, State};

use crate::{BlankData, BlankError, BlankState, BlankStateDiff};

/// Ensure OpSpec for the blank state.
#[derive(Debug)]
pub struct BlankEnsureOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> EnsureOpSpec for BlankEnsureOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = BlankData<'op, Id>;
    type Error = BlankError;
    type StateDiff = BlankStateDiff;
    type StateLogical = BlankState;
    type StatePhysical = Nothing;

    async fn check(
        _blank_data: BlankData<'_, Id>,
        _state_current: &State<BlankState, Nothing>,
        _state_desired: &State<BlankState, Nothing>,
        diff: &BlankStateDiff,
    ) -> Result<OpCheckStatus, BlankError> {
        let op_check_status = match *diff {
            BlankStateDiff::InSync { .. } => OpCheckStatus::ExecNotRequired,
            BlankStateDiff::Added { .. } | BlankStateDiff::OutOfSync { .. } => {
                #[cfg(not(feature = "output_progress"))]
                {
                    OpCheckStatus::ExecRequired
                }
                #[cfg(feature = "output_progress")]
                {
                    let progress_limit = ProgressLimit::Steps(1);
                    OpCheckStatus::ExecRequired { progress_limit }
                }
            }
        };

        Ok(op_check_status)
    }

    async fn exec_dry(
        _op_ctx: OpCtx<'_>,
        _blank_data: BlankData<'_, Id>,
        _state_current: &State<BlankState, Nothing>,
        _state_desired: &State<BlankState, Nothing>,
        _diff: &BlankStateDiff,
    ) -> Result<Nothing, BlankError> {
        Ok(Nothing)
    }

    async fn exec(
        _op_ctx: OpCtx<'_>,
        mut blank_data: BlankData<'_, Id>,
        _state_current: &State<BlankState, Nothing>,
        _state_desired: &State<BlankState, Nothing>,
        _diff: &BlankStateDiff,
    ) -> Result<Nothing, BlankError> {
        let params = blank_data.params_mut();
        **params.dest_mut() = Some(**params.src());

        Ok(Nothing)
    }
}
