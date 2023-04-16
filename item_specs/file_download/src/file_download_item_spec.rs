use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, state::FetchedOpt, ApplyCheck, FnCtx, ItemSpec, ItemSpecId, State},
    resources::{resources::ts::Empty, Resources},
};

use crate::{
    ETag, FileDownloadApplyFns, FileDownloadData, FileDownloadError, FileDownloadParams,
    FileDownloadState, FileDownloadStateCurrentFn, FileDownloadStateDesiredFn,
    FileDownloadStateDiff, FileDownloadStateDiffFn,
};

/// Item spec for downloading a file.
///
/// The `Id` type parameter is needed for each file download params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different file download
///   parameters from each other.
#[derive(Debug)]
pub struct FileDownloadItemSpec<Id> {
    /// ID of the item spec to download the file.
    item_spec_id: ItemSpecId,
    /// Marker for unique download parameters type.
    marker: PhantomData<Id>,
}

impl<Id> Clone for FileDownloadItemSpec<Id> {
    fn clone(&self) -> Self {
        Self {
            item_spec_id: self.item_spec_id.clone(),
            marker: PhantomData,
        }
    }
}

impl<Id> FileDownloadItemSpec<Id> {
    /// Returns a new `FileDownloadItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self {
            item_spec_id,
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for FileDownloadItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'exec> = FileDownloadData<'exec, Id>;
    type Error = FileDownloadError;
    type Params<'exec> = FileDownloadParams<Id>;
    type State = State<FileDownloadState, FetchedOpt<ETag>>;
    type StateDiff = FileDownloadStateDiff;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), FileDownloadError> {
        resources.insert::<reqwest::Client>(reqwest::Client::new());

        Ok(())
    }

    async fn try_state_current(
        fn_ctx: FnCtx<'_>,
        params_partial: Option<&Self::Params<'_>>,
        data: FileDownloadData<'_, Id>,
    ) -> Result<Option<Self::State>, FileDownloadError> {
        FileDownloadStateCurrentFn::try_state_current(fn_ctx, params_partial, data).await
    }

    async fn state_current(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: FileDownloadData<'_, Id>,
    ) -> Result<Self::State, FileDownloadError> {
        FileDownloadStateCurrentFn::state_current(fn_ctx, params, data).await
    }

    async fn try_state_desired(
        fn_ctx: FnCtx<'_>,
        params_partial: Option<&Self::Params<'_>>,
        data: FileDownloadData<'_, Id>,
    ) -> Result<Option<Self::State>, FileDownloadError> {
        FileDownloadStateDesiredFn::try_state_desired(fn_ctx, params_partial, data).await
    }

    async fn state_desired(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: FileDownloadData<'_, Id>,
    ) -> Result<Self::State, FileDownloadError> {
        FileDownloadStateDesiredFn::state_desired(fn_ctx, params, data).await
    }

    async fn state_diff(
        _params_partial: Option<&Self::Params<'_>>,
        _data: Self::Data<'_>,
        state_a: &Self::State,
        state_b: &Self::State,
    ) -> Result<Self::StateDiff, FileDownloadError> {
        FileDownloadStateDiffFn::state_diff(state_a, state_b).await
    }

    async fn state_clean(
        params_partial: Option<&Self::Params<'_>>,
        _data: Self::Data<'_>,
    ) -> Result<Self::State, FileDownloadError> {
        let path = params_partial
            .map(|params| params.dest())
            .unwrap_or_else(|| todo!("Redesign FileDownloadState"))
            .to_path_buf();
        let state = State::new(FileDownloadState::None { path }, FetchedOpt::Tbd);
        Ok(state)
    }

    async fn apply_check(
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<ApplyCheck, Self::Error> {
        FileDownloadApplyFns::<Id>::apply_check(params, data, state_current, state_target, diff)
            .await
    }

    async fn apply_dry(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        FileDownloadApplyFns::<Id>::apply_dry(
            fn_ctx,
            params,
            data,
            state_current,
            state_target,
            diff,
        )
        .await
    }

    async fn apply(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        FileDownloadApplyFns::<Id>::apply(fn_ctx, params, data, state_current, state_target, diff)
            .await
    }
}
