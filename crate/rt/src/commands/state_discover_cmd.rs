use peace_rt_model::Error;
use std::marker::PhantomData;

use peace_resources::{
    resources_type_state::{SetUp, WithStatesCurrentAndDesired},
    Resources,
};
use peace_rt_model::CmdContext;

use crate::{StateCurrentCmd, StateDesiredCmd};

#[derive(Debug)]
pub struct StateDiscoverCmd<E>(PhantomData<E>);

impl<E> StateDiscoverCmd<E>
where
    E: std::error::Error + From<Error> + Send,
{
    /// Runs [`StateCurrentFnSpec`]` and `[`StateDesiredFnSpec`]`::`[`exec`] for
    /// each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`] and [`StatesDesired`], and will be serialized to
    /// `{profile_dir}/states.yaml` and `{profile_dir}/states_desired.yaml`.
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StatesCurrent`]: peace_resources::StatesCurrent
    /// [`StatesDesired`]: peace_resources::StatesDesired
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    pub async fn exec(
        cmd_context: CmdContext<'_, SetUp, E>,
    ) -> Result<CmdContext<WithStatesCurrentAndDesired, E>, E> {
        let (workspace, item_spec_graph, mut resources) = cmd_context.into_inner();
        let states = StateCurrentCmd::exec_internal(item_spec_graph, &mut resources).await?;
        let states_desired =
            StateDesiredCmd::exec_internal(item_spec_graph, &mut resources).await?;

        let resources =
            Resources::<WithStatesCurrentAndDesired>::from((resources, states, states_desired));

        let cmd_context = CmdContext::from((workspace, item_spec_graph, resources));
        Ok(cmd_context)
    }
}
