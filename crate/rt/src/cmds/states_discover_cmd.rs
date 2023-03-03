use std::{fmt::Debug, marker::PhantomData};

use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{
    resources::ts::{SetUp, WithStatesCurrentAndDesired},
    Resources,
};
use peace_rt_model::{params::ParamsKeys, Error};

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
    pub async fn exec(
        mut cmd_ctx: CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, WithStatesCurrentAndDesired>>, E>
    {
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
}

impl<E, O, PKeys> Default for StatesDiscoverCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
