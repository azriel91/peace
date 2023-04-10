use std::{fmt::Debug, marker::PhantomData};

use futures::{StreamExt, TryStreamExt};
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{
    internal::StateDiffsMut,
    resources::ts::SetUp,
    states::{StateDiffs, States},
    Resources,
};
use peace_rt_model::{output::OutputWrite, params::ParamsKeys, Error, Flow};

use crate::cmds::sub::{StatesDesiredReadCmd, StatesSavedReadCmd};

#[derive(Debug)]
pub struct DiffCmd<E>(PhantomData<E>);

impl<E> DiffCmd<E>
where
    E: std::error::Error + From<Error> + Send + 'static,
{
    /// Returns the [`state_diff`]`s between the saved current and desired
    /// states.
    ///
    /// Both current and desired states must have been discovered prior to
    /// running this. See [`StatesDiscoverCmd::current_and_desired`].
    ///
    /// [`state_diff`]: peace_cfg::ItemSpec::state_diff
    /// [`StatesDiscoverCmd::current_and_desired`]: crate::cmds::StatesDiscoverCmd::current_and_desired
    pub async fn current_and_desired<O, PKeys>(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<StateDiffs, E>
    where
        PKeys: ParamsKeys + 'static,
        O: OutputWrite<E>,
    {
        let states_a = StatesSavedReadCmd::exec(cmd_ctx).await?;
        let states_b = StatesDesiredReadCmd::exec(cmd_ctx).await?;

        let SingleProfileSingleFlowView {
            flow, resources, ..
        } = cmd_ctx.view();

        Self::diff_any(flow, resources, &states_a, &states_b).await
    }

    /// Returns the [`state_diff`]` for each [`ItemSpec`].
    ///
    /// This does not take in `CmdCtx` as it may be used by both
    /// `SingleProfileSingleFlow` and `MultiProfileSingleFlow`
    /// commands.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`state_diff`]: peace_cfg::ItemSpec::state_diff
    pub async fn diff_any<StatesTsA, StatesTsB>(
        flow: &Flow<E>,
        resources: &Resources<SetUp>,
        states_a: &States<StatesTsA>,
        states_b: &States<StatesTsB>,
    ) -> Result<StateDiffs, E> {
        let resources_ref = &*resources;
        let state_diffs = {
            let state_diffs_mut = flow
                .graph()
                .stream()
                .map(Result::<_, E>::Ok)
                .try_filter_map(|item_spec| async move {
                    let state_diff_opt = item_spec
                        .state_diff_exec(resources_ref, states_a, states_b)
                        .await?;

                    Ok(state_diff_opt.map(|state_diff| (item_spec.id().clone(), state_diff)))
                })
                .try_collect::<StateDiffsMut>()
                .await?;

            StateDiffs::from(state_diffs_mut)
        };

        Ok(state_diffs)
    }
}

impl<E> Default for DiffCmd<E> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
