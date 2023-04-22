use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ApplyCheck, FnCtx, ItemSpec, ItemSpecId},
    resources::{resources::ts::Empty, Resources},
};

use crate::{BlankApplyFns, BlankData, BlankError, BlankParams, BlankState, BlankStateDiff};

/// Item spec for copying a number.
///
/// The `Id` type parameter is needed for each blank params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different blank parameters
///   from each other.
#[derive(Debug)]
pub struct BlankItemSpec<Id> {
    /// ID of the blank item spec.
    item_spec_id: ItemSpecId,
    /// Marker for unique blank parameters type.
    marker: PhantomData<Id>,
}

impl<Id> BlankItemSpec<Id> {
    /// Returns a new `BlankItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self {
            item_spec_id,
            marker: PhantomData,
        }
    }
}

impl<Id> Clone for BlankItemSpec<Id> {
    fn clone(&self) -> Self {
        Self {
            item_spec_id: self.item_spec_id.clone(),
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for BlankItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'exec> = BlankData<'exec, Id>;
    type Error = BlankError;
    type Params<'exec> = BlankParams<Id>;
    type State = BlankState;
    type StateDiff = BlankStateDiff;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, _resources: &mut Resources<Empty>) -> Result<(), BlankError> {
        Ok(())
    }

    async fn try_state_current(
        fn_ctx: FnCtx<'_>,
        params_partial: Option<&Self::Params<'_>>,
        data: BlankData<'_, Id>,
    ) -> Result<Option<Self::State>, BlankError> {
        if let Some(params) = params_partial {
            Self::state_current(fn_ctx, params, data).await.map(Some)
        } else {
            Ok(None)
        }
    }

    async fn state_current(
        _fn_ctx: FnCtx<'_>,
        _params: &Self::Params<'_>,
        data: BlankData<'_, Id>,
    ) -> Result<Self::State, BlankError> {
        let current = BlankState(data.params().dest().0);

        let state = current;

        Ok(state)
    }

    async fn try_state_desired(
        fn_ctx: FnCtx<'_>,
        params_partial: Option<&Self::Params<'_>>,
        data: BlankData<'_, Id>,
    ) -> Result<Option<Self::State>, BlankError> {
        if let Some(params) = params_partial {
            Self::state_desired(fn_ctx, params, data).await.map(Some)
        } else {
            Ok(None)
        }
    }

    async fn state_desired(
        _fn_ctx: FnCtx<'_>,
        _params: &Self::Params<'_>,
        data: BlankData<'_, Id>,
    ) -> Result<Self::State, BlankError> {
        let params = data.params();
        Ok(BlankState(Some(**params.src())))
    }

    async fn state_diff(
        _params_partial: Option<&Self::Params<'_>>,
        _data: Self::Data<'_>,
        state_current: &BlankState,
        state_desired: &BlankState,
    ) -> Result<Self::StateDiff, BlankError> {
        let diff = match (state_current, state_desired) {
            (BlankState(Some(current)), BlankState(Some(desired))) if current == desired => {
                BlankStateDiff::InSync { value: *current }
            }
            (BlankState(Some(current)), BlankState(Some(desired))) => BlankStateDiff::OutOfSync {
                diff: i64::from(desired - current),
            },
            (BlankState(None), BlankState(Some(desired))) => {
                BlankStateDiff::Added { value: *desired }
            }
            (BlankState(_), BlankState(None)) => unreachable!("desired state is always Some"),
        };

        Ok(diff)
    }

    async fn state_clean(
        _params_partial: Option<&Self::Params<'_>>,
        _data: Self::Data<'_>,
    ) -> Result<BlankState, BlankError> {
        Ok(BlankState(None))
    }

    async fn apply_check(
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<ApplyCheck, Self::Error> {
        BlankApplyFns::<Id>::apply_check(params, data, state_current, state_target, diff).await
    }

    async fn apply_dry(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        BlankApplyFns::<Id>::apply_dry(fn_ctx, params, data, state_current, state_target, diff)
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
        BlankApplyFns::<Id>::apply(fn_ctx, params, data, state_current, state_target, diff).await
    }
}
