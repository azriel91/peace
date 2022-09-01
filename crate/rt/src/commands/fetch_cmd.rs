use peace_rt_model::Error;
use std::marker::PhantomData;

use peace_resources::{
    resources_type_state::{SetUp, WithStatesCurrentAndDesired},
    Resources,
};
use peace_rt_model::CmdContext;

use crate::{StateCurrentCmd, StateDesiredCmd};

#[derive(Debug)]
pub struct FetchCmd<E>(PhantomData<E>);

impl<E> FetchCmd<E>
where
    E: std::error::Error + From<Error> + Send,
{
    /// Runs [`StateCurrentFnSpec`]` and `[`StateDesiredFnSpec`]`::`[`exec`] for
    /// each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`States`] and [`StatesDesired`], and will be serialized to
    /// `{profile_dir}/states.yaml` and `{profile_dir}/states_desired.yaml`.
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`States`]: peace_resources::States
    /// [`StatesDesired`]: peace_resources::StatesDesired
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    pub async fn exec(
        cmd_context: CmdContext<'_, SetUp, E>,
    ) -> Result<CmdContext<WithStatesCurrentAndDesired, E>, E> {
        let (workspace, item_spec_graph, resources) = cmd_context.into_inner();
        let states = StateCurrentCmd::exec_internal(item_spec_graph, &resources).await?;
        let states_desired = StateDesiredCmd::exec_internal(item_spec_graph, &resources).await?;

        let resources =
            Resources::<WithStatesCurrentAndDesired>::from((resources, states, states_desired));

        let cmd_context = CmdContext::from((workspace, item_spec_graph, resources));
        Ok(cmd_context)
    }
}
