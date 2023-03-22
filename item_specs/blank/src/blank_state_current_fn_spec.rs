use std::marker::PhantomData;

use peace::cfg::{async_trait, OpCtx, TryFnSpec};

use crate::{BlankData, BlankError, BlankState};

/// Reads the current state of the blank state.
#[derive(Debug)]
pub struct BlankStateCurrentFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> TryFnSpec for BlankStateCurrentFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = BlankData<'op, Id>;
    type Error = BlankError;
    type Output = BlankState;

    async fn try_exec(
        op_ctx: OpCtx<'_>,
        blank_data: BlankData<'_, Id>,
    ) -> Result<Option<Self::Output>, BlankError> {
        Self::exec(op_ctx, blank_data).await.map(Some)
    }

    async fn exec(
        _op_ctx: OpCtx<'_>,
        blank_data: BlankData<'_, Id>,
    ) -> Result<Self::Output, BlankError> {
        let current = BlankState(blank_data.params().dest().0);

        let state = current;

        Ok(state)
    }
}
