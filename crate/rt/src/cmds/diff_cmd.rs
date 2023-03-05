use std::{fmt::Debug, marker::PhantomData};

use futures::{StreamExt, TryStreamExt};
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{internal::StateDiffsMut, resources::ts::SetUp, states::StateDiffs};
use peace_rt_model::{output::OutputWrite, params::ParamsKeys, Error};

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
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<StateDiffs, E> {
        let states_saved = StatesSavedReadCmd::<E, O, PKeys>::exec(cmd_ctx).await?;
        let states_desired = StatesDesiredReadCmd::<E, O, PKeys>::exec(cmd_ctx).await?;

        let SingleProfileSingleFlowView {
            flow, resources, ..
        } = cmd_ctx.scope_mut().view();
        let item_spec_graph = flow.graph();

        let resources_ref = &*resources;
        let states_saved_ref = &states_saved;
        let states_desired_ref = &states_desired;
        let state_diffs = {
            let state_diffs_mut = item_spec_graph
                .stream()
                .map(Result::<_, E>::Ok)
                .and_then(|item_spec| async move {
                    Ok((
                        item_spec.id().clone(),
                        item_spec
                            .state_diff_exec_with_states_saved(
                                resources_ref,
                                states_saved_ref,
                                states_desired_ref,
                            )
                            .await?,
                    ))
                })
                .try_collect::<StateDiffsMut>()
                .await?;

            StateDiffs::from(state_diffs_mut)
        };

        Ok(state_diffs)
    }
}

impl<E, O, PKeys> Default for DiffCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
