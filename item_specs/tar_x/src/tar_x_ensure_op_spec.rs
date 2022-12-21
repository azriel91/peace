use std::marker::PhantomData;

use peace::cfg::state::Nothing;

use peace::cfg::{async_trait, EnsureOpSpec, OpCheckStatus, State};

use crate::{FileMetadatas, TarXData, TarXError, TarXStateDiff};

/// Ensure OpSpec for the tar to extract.
#[derive(Debug)]
pub struct TarXEnsureOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> EnsureOpSpec for TarXEnsureOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = TarXData<'op, Id>;
    type Error = TarXError;
    type StateDiff = TarXStateDiff;
    type StateLogical = FileMetadatas;
    type StatePhysical = Nothing;

    async fn check(
        _tar_x_data: TarXData<'_, Id>,
        _file_state_current: &State<FileMetadatas, Nothing>,
        _file_state_desired: &FileMetadatas,
        _diff: &TarXStateDiff,
    ) -> Result<OpCheckStatus, TarXError> {
        todo!();
    }

    async fn exec_dry(
        _tar_x_data: TarXData<'_, Id>,
        _state: &State<FileMetadatas, Nothing>,
        _file_state_desired: &FileMetadatas,
        _diff: &TarXStateDiff,
    ) -> Result<Nothing, TarXError> {
        Ok(Nothing)
    }

    async fn exec(
        _tar_x_data: TarXData<'_, Id>,
        _state: &State<FileMetadatas, Nothing>,
        _file_state_desired: &FileMetadatas,
        _diff: &TarXStateDiff,
    ) -> Result<Nothing, TarXError> {
        todo!();
    }
}
