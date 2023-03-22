use std::marker::PhantomData;

use peace::cfg::{async_trait, OpCtx, TryFnSpec};

use crate::{BlankData, BlankError, BlankState};

/// Reads the desired state of the blank state.
#[derive(Debug)]
pub struct BlankStateDesiredFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> TryFnSpec for BlankStateDesiredFnSpec<Id>
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
        let params = blank_data.params();
        Ok(BlankState(Some(**params.src())))
    }
}
