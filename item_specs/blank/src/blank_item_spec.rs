use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, FnCtx, ItemSpec, ItemSpecId, OpCheckStatus},
    resources::{resources::ts::Empty, Resources},
};

use crate::{BlankApplyFns, BlankData, BlankError, BlankState, BlankStateDiff};

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
    type Data<'op> = BlankData<'op, Id>;
    type Error = BlankError;
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
        data: BlankData<'_, Id>,
    ) -> Result<Option<Self::State>, BlankError> {
        Self::state_current(fn_ctx, data).await.map(Some)
    }

    async fn state_current(
        _fn_ctx: FnCtx<'_>,
        data: BlankData<'_, Id>,
    ) -> Result<Self::State, BlankError> {
        let current = BlankState(data.params().dest().0);

        let state = current;

        Ok(state)
    }

    async fn try_state_desired(
        fn_ctx: FnCtx<'_>,
        data: BlankData<'_, Id>,
    ) -> Result<Option<Self::State>, BlankError> {
        Self::state_desired(fn_ctx, data).await.map(Some)
    }

    async fn state_desired(
        _fn_ctx: FnCtx<'_>,
        data: BlankData<'_, Id>,
    ) -> Result<Self::State, BlankError> {
        let params = data.params();
        Ok(BlankState(Some(**params.src())))
    }

    async fn state_diff(
        _data: BlankData<'_, Id>,
        blank_state_current: &BlankState,
        blank_state_desired: &BlankState,
    ) -> Result<Self::StateDiff, BlankError> {
        let diff = match (blank_state_current, blank_state_desired) {
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

    async fn state_clean(_: Self::Data<'_>) -> Result<BlankState, BlankError> {
        Ok(BlankState(None))
    }

    async fn apply_check(
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<OpCheckStatus, Self::Error> {
        BlankApplyFns::apply_check(data, state_current, state_target, diff).await
    }

    async fn apply_dry(
        fn_ctx: FnCtx<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        BlankApplyFns::apply_dry(fn_ctx, data, state_current, state_target, diff).await
    }

    async fn apply(
        fn_ctx: FnCtx<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        BlankApplyFns::apply(fn_ctx, data, state_current, state_target, diff).await
    }
}
