use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ApplyCheck, FnCtx, ItemSpec, ItemSpecId, State},
    resources::{resources::ts::Empty, Resources},
};

use crate::{
    ShCmdApplyFns, ShCmdData, ShCmdError, ShCmdExecutionRecord, ShCmdExecutor, ShCmdParams,
    ShCmdState, ShCmdStateDiff, ShCmdStateDiffFn,
};

/// Item spec for executing a shell command.
///
/// The `Id` type parameter is needed for each command execution params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different command execution
///   parameters from each other.
#[derive(Debug)]
pub struct ShCmdItemSpec<Id> {
    /// ID to easily tell what the item spec command is for.
    item_spec_id: ItemSpecId,
    /// Parameters to insert into `resources` in [`ItemSpec::setup`].
    sh_cmd_params: Option<ShCmdParams<Id>>,
    /// Marker for unique command execution parameters type.
    marker: PhantomData<Id>,
}

impl<Id> Clone for ShCmdItemSpec<Id> {
    fn clone(&self) -> Self {
        Self {
            item_spec_id: self.item_spec_id.clone(),
            sh_cmd_params: self.sh_cmd_params.clone(),
            marker: PhantomData,
        }
    }
}

impl<Id> ShCmdItemSpec<Id> {
    /// Returns a new `ShCmdItemSpec`.
    ///
    /// # Parameters
    ///
    /// * `item_spec_id`: ID of this `ShCmdItemSpec`.
    /// * `sh_cmd_params`: Parameters to insert into `Resources`.
    pub fn new(item_spec_id: ItemSpecId, sh_cmd_params: Option<ShCmdParams<Id>>) -> Self {
        Self {
            item_spec_id,
            sh_cmd_params,
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for ShCmdItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'exec> = ShCmdData<'exec, Id>;
    type Error = ShCmdError;
    type Params<'exec> = ShCmdParams<Id>;
    type State = State<ShCmdState<Id>, ShCmdExecutionRecord>;
    type StateDiff = ShCmdStateDiff;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), ShCmdError> {
        if let Some(sh_cmd_params) = self.sh_cmd_params.clone() {
            resources.insert(sh_cmd_params);
        }

        Ok(())
    }

    async fn try_state_current(
        fn_ctx: FnCtx<'_>,
        params_partial: Option<&Self::Params<'_>>,
        data: ShCmdData<'_, Id>,
    ) -> Result<Option<Self::State>, ShCmdError> {
        if let Some(params) = params_partial {
            Self::state_current(fn_ctx, params, data).await.map(Some)
        } else {
            Ok(None)
        }
    }

    async fn state_current(
        _fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        _data: ShCmdData<'_, Id>,
    ) -> Result<Self::State, ShCmdError> {
        let state_current_sh_cmd = params.state_current_sh_cmd();
        ShCmdExecutor::exec(state_current_sh_cmd).await
    }

    async fn try_state_desired(
        fn_ctx: FnCtx<'_>,
        params_partial: Option<&Self::Params<'_>>,
        data: ShCmdData<'_, Id>,
    ) -> Result<Option<Self::State>, ShCmdError> {
        if let Some(params) = params_partial {
            Self::state_desired(fn_ctx, params, data).await.map(Some)
        } else {
            Ok(None)
        }
    }

    async fn state_desired(
        _fn_ctx: FnCtx<'_>,
        _params: &Self::Params<'_>,
        data: ShCmdData<'_, Id>,
    ) -> Result<Self::State, ShCmdError> {
        let state_desired_sh_cmd = data.sh_cmd_params().state_desired_sh_cmd();
        // Maybe we should support reading different exit statuses for an `Ok(None)`
        // value.
        ShCmdExecutor::exec(state_desired_sh_cmd).await
    }

    async fn state_diff(
        _params_partial: Option<&Self::Params<'_>>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_desired: &Self::State,
    ) -> Result<Self::StateDiff, ShCmdError> {
        ShCmdStateDiffFn::state_diff(data, state_current, state_desired).await
    }

    async fn state_clean(
        _params_partial: Option<&Self::Params<'_>>,
        data: Self::Data<'_>,
    ) -> Result<Self::State, ShCmdError> {
        let state_clean_sh_cmd = data.sh_cmd_params().state_clean_sh_cmd();
        ShCmdExecutor::exec(state_clean_sh_cmd).await
    }

    async fn apply_check(
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<ApplyCheck, Self::Error> {
        ShCmdApplyFns::<Id>::apply_check(params, data, state_current, state_target, diff).await
    }

    async fn apply_dry(
        fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        data: Self::Data<'_>,
        state_current: &Self::State,
        state_target: &Self::State,
        diff: &Self::StateDiff,
    ) -> Result<Self::State, Self::Error> {
        ShCmdApplyFns::<Id>::apply_dry(fn_ctx, params, data, state_current, state_target, diff)
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
        ShCmdApplyFns::<Id>::apply(fn_ctx, params, data, state_current, state_target, diff).await
    }
}
