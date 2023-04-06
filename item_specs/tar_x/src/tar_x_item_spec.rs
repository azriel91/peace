use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ItemSpec, ItemSpecId, OpCtx},
    resources::{resources::ts::Empty, Resources},
};

use crate::{
    FileMetadatas, TarXApplyOpSpec, TarXData, TarXError, TarXStateCurrentFn,
    TarXStateDesiredFnSpec, TarXStateDiff, TarXStateDiffFnSpec,
};

/// Item spec for extracting a tar file.
///
/// The `Id` type parameter is needed for each tar extraction params to be a
/// distinct type.
///
/// The following use cases are intended to be supported:
///
/// * A pristine directory with only the tar's contents and nothing else (in
///   progress).
/// * Extraction of a tar over an existing directory (not yet implemented).
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different tar extraction
///   parameters from each other.
#[derive(Debug)]
pub struct TarXItemSpec<Id> {
    /// ID of the item spec to extract the tar.
    item_spec_id: ItemSpecId,
    /// Marker for unique tar extraction parameters type.
    marker: PhantomData<Id>,
}

impl<Id> Clone for TarXItemSpec<Id> {
    fn clone(&self) -> Self {
        Self {
            item_spec_id: self.item_spec_id.clone(),
            marker: PhantomData,
        }
    }
}

impl<Id> TarXItemSpec<Id> {
    /// Returns a new `TarXItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self {
            item_spec_id,
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for TarXItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type ApplyOpSpec = TarXApplyOpSpec<Id>;
    type Data<'op> = TarXData<'op, Id>;
    type Error = TarXError;
    type State = FileMetadatas;
    type StateDiff = TarXStateDiff;
    type StateDiffFnSpec = TarXStateDiffFnSpec;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, _resources: &mut Resources<Empty>) -> Result<(), TarXError> {
        Ok(())
    }

    async fn try_state_current(
        op_ctx: OpCtx<'_>,
        data: TarXData<'_, Id>,
    ) -> Result<Option<Self::State>, TarXError> {
        Self::state_current(op_ctx, data).await.map(Some)
    }

    async fn state_current(
        op_ctx: OpCtx<'_>,
        data: TarXData<'_, Id>,
    ) -> Result<Self::State, TarXError> {
        TarXStateCurrentFn::state_current(op_ctx, data).await
    }

    async fn try_state_desired(
        op_ctx: OpCtx<'_>,
        data: TarXData<'_, Id>,
    ) -> Result<Option<Self::State>, TarXError> {
        TarXStateDesiredFnSpec::try_state_desired(op_ctx, data).await
    }

    async fn state_desired(
        op_ctx: OpCtx<'_>,
        data: TarXData<'_, Id>,
    ) -> Result<Self::State, TarXError> {
        TarXStateDesiredFnSpec::state_desired(op_ctx, data).await
    }

    async fn state_clean(_: Self::Data<'_>) -> Result<Self::State, TarXError> {
        Ok(FileMetadatas::default())
    }
}
