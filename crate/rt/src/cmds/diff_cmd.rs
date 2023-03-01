use std::{fmt::Debug, marker::PhantomData};

use futures::{StreamExt, TryStreamExt};
use peace_cmd::{
    ctx::{CmdCtx, CmdCtxView},
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{
    internal::StateDiffsMut,
    resources::ts::{SetUp, WithStatesSavedAndDesired, WithStatesSavedDiffs},
    states::StateDiffs,
    Resources,
};
use peace_rt_model::{cmd_context_params::ParamsKeys, output::OutputWrite, Error};

use crate::cmds::sub::{StatesDesiredReadCmd, StatesSavedReadCmd};

#[derive(Debug)]
pub struct DiffCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> DiffCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
    O: OutputWrite<E>,
{
    /// Runs [`StateDiffFnSpec`]` for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`], [`StatesDesired`], and [`StateDiffs`].
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StatesCurrent`]: peace_resources::StatesCurrent
    /// [`StatesDesired`]: peace_resources::StatesDesired
    /// [`StatesRw`]: peace_resources::StatesRw
    /// [`StateDiffFnSpec`]: peace_cfg::ItemSpec::StateDiffFnSpec
    /// [`StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    pub async fn exec(
        cmd_ctx: CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, SetUp>, PKeys>,
    ) -> Result<CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, WithStatesSavedDiffs>, PKeys>, E>
    {
        let cmd_ctx_result = Self::exec_internal_with_states_saved(cmd_ctx).await;
        match cmd_ctx_result {
            Ok(mut cmd_ctx) => {
                {
                    let CmdCtxView { output, scope, .. } = cmd_ctx.view();
                    let resources = scope.resources();
                    let state_diffs = resources.borrow::<StateDiffs>();
                    output.present(&*state_diffs).await?;
                }

                Ok(cmd_ctx)
            }
            Err(e) => Err(e),
        }
    }

    /// Returns `StateDiffs` between the saved and desired states on disk.
    ///
    /// This also updates `Resources` from `SetUp` to
    /// `WithStatesCurrentAndDesired`.
    pub(crate) async fn exec_internal_with_states_saved(
        mut cmd_ctx: CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, SetUp>, PKeys>,
    ) -> Result<CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, WithStatesSavedDiffs>, PKeys>, E>
    {
        let SingleProfileSingleFlowView {
            resources,
            states_type_regs,
            ..
        } = cmd_ctx.scope_mut().view();

        let states_saved = StatesSavedReadCmd::<E, O, PKeys>::exec_internal(
            resources,
            states_type_regs.states_current_type_reg(),
        )
        .await?;
        let states_desired = StatesDesiredReadCmd::<E, O, PKeys>::exec_internal(
            resources,
            states_type_regs.states_desired_type_reg(),
        )
        .await?;

        let mut cmd_ctx = cmd_ctx.resources_update(|resources| {
            Resources::<WithStatesSavedAndDesired>::from((resources, states_saved, states_desired))
        });

        let SingleProfileSingleFlowView {
            flow, resources, ..
        } = cmd_ctx.scope_mut().view();
        let item_spec_graph = flow.graph();

        let resources_ref = &*resources;
        let state_diffs = {
            let state_diffs_mut = item_spec_graph
                .stream()
                .map(Result::<_, E>::Ok)
                .and_then(|item_spec| async move {
                    Ok((
                        item_spec.id().clone(),
                        item_spec
                            .state_diff_exec_with_states_saved(resources_ref)
                            .await?,
                    ))
                })
                .try_collect::<StateDiffsMut>()
                .await?;

            StateDiffs::from(state_diffs_mut)
        };

        let cmd_ctx = cmd_ctx.resources_update(|resources| {
            Resources::<WithStatesSavedDiffs>::from((resources, state_diffs))
        });
        Ok(cmd_ctx)
    }
}

impl<E, O, PKeys> Default for DiffCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
