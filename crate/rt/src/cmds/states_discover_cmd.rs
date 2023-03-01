use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{
    resources::ts::{SetUp, WithStatesCurrentAndDesired},
    Resources,
};
use peace_rt_model::{cmd::CmdContext, cmd_context_params::ParamsKeys, Error};

use crate::cmds::sub::{StatesCurrentDiscoverCmd, StatesDesiredDiscoverCmd};

#[derive(Debug)]
pub struct StatesDiscoverCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> StatesDiscoverCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Runs [`StateCurrentFnSpec`]` and
    /// `[`StateDesiredFnSpec`]`::`[`try_exec`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`] and [`StatesDesired`], and will be serialized to
    /// `{profile_dir}/states.yaml` and `{profile_dir}/states_desired.yaml`.
    ///
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StatesCurrent`]: peace_resources::StatesCurrent
    /// [`StatesDesired`]: peace_resources::StatesDesired
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    pub async fn exec_v2(
        mut cmd_ctx: CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, SetUp>, PKeys>,
    ) -> Result<
        CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, WithStatesCurrentAndDesired>, PKeys>,
        E,
    > {
        let SingleProfileSingleFlowView {
            flow, resources, ..
        } = cmd_ctx.scope_mut().view();
        let item_spec_graph = flow.graph();

        let states_current =
            StatesCurrentDiscoverCmd::<E, O, PKeys>::exec_internal(item_spec_graph, resources)
                .await?;
        let states_desired =
            StatesDesiredDiscoverCmd::<E, O, PKeys>::exec_internal(item_spec_graph, resources)
                .await?;

        let cmd_ctx = cmd_ctx.resources_update(|resources| {
            Resources::<WithStatesCurrentAndDesired>::from((
                resources,
                states_current,
                states_desired,
            ))
        });
        Ok(cmd_ctx)
    }

    /// Runs [`StateCurrentFnSpec`]` and
    /// `[`StateDesiredFnSpec`]`::`[`try_exec`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`] and [`StatesDesired`], and will be serialized to
    /// `{profile_dir}/states.yaml` and `{profile_dir}/states_desired.yaml`.
    ///
    /// [`try_exec`]: peace_cfg::TryFnSpec::try_exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StatesCurrent`]: peace_resources::StatesCurrent
    /// [`StatesDesired`]: peace_resources::StatesDesired
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    pub async fn exec(
        cmd_context: CmdContext<'_, E, O, SetUp, PKeys>,
    ) -> Result<CmdContext<'_, E, O, WithStatesCurrentAndDesired, PKeys>, E> {
        let CmdContext {
            workspace,
            item_spec_graph,
            output,
            mut resources,
            params_type_regs,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            ..
        } = cmd_context;
        let states_current =
            StatesCurrentDiscoverCmd::<E, O, PKeys>::exec_internal(item_spec_graph, &mut resources)
                .await?;
        let states_desired =
            StatesDesiredDiscoverCmd::<E, O, PKeys>::exec_internal(item_spec_graph, &mut resources)
                .await?;

        let resources = Resources::<WithStatesCurrentAndDesired>::from((
            resources,
            states_current,
            states_desired,
        ));

        let cmd_context = CmdContext::from((
            workspace,
            item_spec_graph,
            output,
            resources,
            params_type_regs,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
        ));
        Ok(cmd_context)
    }
}

impl<E, O, PKeys> Default for StatesDiscoverCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
