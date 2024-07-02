use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ApplyCheck, FnCtx, Item, ItemId, State},
    params::Params,
    resource_rt::{resources::ts::Empty, Resources},
};

use crate::{
    ShCmdApplyFns, ShCmdData, ShCmdError, ShCmdExecutionRecord, ShCmdExecutor, ShCmdParams,
    ShCmdState, ShCmdStateDiff, ShCmdStateDiffFn,
};

/// Item for executing a shell command.
///
/// The `Id` type parameter is needed for each command execution params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different command execution
///   parameters from each other.
#[derive(Debug)]
pub struct ShCmdItem<Id> {
    /// ID to easily tell what the item command is for.
    item_id: ItemId,
    /// Marker for unique command execution parameters type.
    marker: PhantomData<Id>,
}

impl<Id> Clone for ShCmdItem<Id> {
    fn clone(&self) -> Self {
        Self {
            item_id: self.item_id.clone(),
            marker: PhantomData,
        }
    }
}

impl<Id> ShCmdItem<Id> {
    /// Returns a new `ShCmdItem`.
    ///
    /// # Parameters
    ///
    /// * `item_id`: ID of this `ShCmdItem`.
    pub fn new(item_id: ItemId) -> Self {
        Self {
            item_id,
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> Item for ShCmdItem<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'exec> = ShCmdData<'exec, Id>;
    type Error = ShCmdError;
    type Params<'exec> = ShCmdParams<Id>;
    type State = State<ShCmdState<Id>, ShCmdExecutionRecord>;
    type StateDiff = ShCmdStateDiff;

    fn id(&self) -> &ItemId {
        &self.item_id
    }

    async fn setup(&self, _resources: &mut Resources<Empty>) -> Result<(), ShCmdError> {
        Ok(())
    }

    async fn try_state_current(
        _fn_ctx: FnCtx<'_>,
        params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: ShCmdData<'_, Id>,
    ) -> Result<Option<Self::State>, ShCmdError> {
        if let Some(state_current_sh_cmd) = params_partial.state_current_sh_cmd() {
            ShCmdExecutor::exec(state_current_sh_cmd).await.map(Some)
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

    async fn try_state_goal(
        _fn_ctx: FnCtx<'_>,
        params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: ShCmdData<'_, Id>,
    ) -> Result<Option<Self::State>, ShCmdError> {
        if let Some(state_goal_sh_cmd) = params_partial.state_goal_sh_cmd() {
            ShCmdExecutor::exec(state_goal_sh_cmd).await.map(Some)
        } else {
            Ok(None)
        }
    }

    async fn state_goal(
        _fn_ctx: FnCtx<'_>,
        params: &Self::Params<'_>,
        _data: ShCmdData<'_, Id>,
    ) -> Result<Self::State, ShCmdError> {
        let state_goal_sh_cmd = params.state_goal_sh_cmd();
        // Maybe we should support reading different exit statuses for an `Ok(None)`
        // value.
        ShCmdExecutor::exec(state_goal_sh_cmd).await
    }

    async fn state_diff(
        params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: Self::Data<'_>,
        state_current: &Self::State,
        state_goal: &Self::State,
    ) -> Result<Self::StateDiff, ShCmdError> {
        let state_diff_sh_cmd =
            params_partial
                .state_diff_sh_cmd()
                .ok_or(ShCmdError::CmdScriptNotResolved {
                    cmd_variant: crate::CmdVariant::StateDiff,
                })?;

        ShCmdStateDiffFn::state_diff(state_diff_sh_cmd.clone(), state_current, state_goal).await
    }

    async fn state_clean(
        params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: Self::Data<'_>,
    ) -> Result<Self::State, ShCmdError> {
        let state_clean_sh_cmd =
            params_partial
                .state_clean_sh_cmd()
                .ok_or(ShCmdError::CmdScriptNotResolved {
                    cmd_variant: crate::CmdVariant::StateClean,
                })?;

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

    #[cfg(feature = "resource_interactions")]
    fn resource_interaction(
        _params_partial: &<Self::Params<'_> as Params>::Partial,
        _data: Self::Data<'_>,
    ) -> peace::resource_model::ItemInteraction {
        use peace::resource_model::{ItemInteractionWithin, ItemLocation};

        ItemInteractionWithin::new(vec![ItemLocation::localhost()]).into()
    }
}
