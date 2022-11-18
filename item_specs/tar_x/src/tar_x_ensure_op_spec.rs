use std::marker::PhantomData;

use peace::cfg::state::Nothing;

use peace::cfg::{async_trait, EnsureOpSpec, OpCheckStatus, State};

use crate::{TarXData, TarXError, TarXState, TarXStateDiff};

/// Ensure OpSpec for the tar to extract.
#[derive(Debug)]
pub struct TarXEnsureOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> EnsureOpSpec for TarXEnsureOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = TarXData<'op, Id>
        where Self: 'op;
    type Error = TarXError;
    type StateDiff = TarXStateDiff;
    type StateLogical = TarXState;
    type StatePhysical = Nothing;

    async fn check(
        _tar_x_data: TarXData<'_, Id>,
        _file_state_current: &State<TarXState, Nothing>,
        _file_state_desired: &TarXState,
        _diff: &TarXStateDiff,
    ) -> Result<OpCheckStatus, TarXError> {
        todo!();
    }

    async fn exec_dry(
        _tar_x_data: TarXData<'_, Id>,
        _state: &State<TarXState, Nothing>,
        _file_state_desired: &TarXState,
        _diff: &TarXStateDiff,
    ) -> Result<Nothing, TarXError> {
        Ok(Nothing)
    }

    async fn exec(
        _tar_x_data: TarXData<'_, Id>,
        _state: &State<TarXState, Nothing>,
        _file_state_desired: &TarXState,
        _diff: &TarXStateDiff,
    ) -> Result<Nothing, TarXError> {
        todo!();
    }
}
