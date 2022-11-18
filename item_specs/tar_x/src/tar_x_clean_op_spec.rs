use std::marker::PhantomData;

use peace::cfg::{async_trait, state::Nothing, CleanOpSpec, OpCheckStatus, State};

use crate::{TarXData, TarXError, TarXState};

/// `CleanOpSpec` for the tar to extract.
#[derive(Debug, Default)]
pub struct TarXCleanOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> CleanOpSpec for TarXCleanOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = TarXData<'op, Id>
        where Self: 'op;
    type Error = TarXError;
    type StateLogical = TarXState;
    type StatePhysical = Nothing;

    async fn check(
        _tar_x_data: TarXData<'_, Id>,
        _state: &State<TarXState, Nothing>,
    ) -> Result<OpCheckStatus, TarXError> {
        todo!()
    }

    async fn exec_dry(
        _tar_x_data: TarXData<'_, Id>,
        _state: &State<TarXState, Nothing>,
    ) -> Result<(), TarXError> {
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn exec(
        _tar_x_data: TarXData<'_, Id>,
        _state: &State<TarXState, Nothing>,
    ) -> Result<(), TarXError> {
        todo!()
    }

    #[cfg(target_arch = "wasm32")]
    async fn exec(
        tar_x_data: TarXData<'_, Id>,
        State {
            logical: file_state,
            ..
        }: &State<TarXState, Nothing>,
    ) -> Result<(), TarXError> {
        todo!()
    }
}
