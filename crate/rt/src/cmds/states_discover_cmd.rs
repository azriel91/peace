use peace_rt_model::Error;
use std::marker::PhantomData;

use peace_resources::{
    resources::ts::{SetUp, WithStatesCurrentAndDesired},
    Resources,
};
use peace_rt_model::CmdContext;

use crate::cmds::sub::{StatesCurrentDiscoverCmd, StatesDesiredDiscoverCmd};

#[derive(Debug)]
pub struct StatesDiscoverCmd<E, O, PO>(PhantomData<(E, O, PO)>);

impl<E, O, PO> StatesDiscoverCmd<E, O, PO>
where
    E: std::error::Error + From<Error> + Send,
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
        cmd_context: CmdContext<'_, E, O, PO, SetUp>,
    ) -> Result<CmdContext<'_, E, O, PO, WithStatesCurrentAndDesired>, E> {
        let (workspace, item_spec_graph, output, progress_output, mut resources, states_type_regs) =
            cmd_context.into_inner();
        let states_current =
            StatesCurrentDiscoverCmd::<E, O, PO>::exec_internal(item_spec_graph, &mut resources)
                .await?;
        let states_desired =
            StatesDesiredDiscoverCmd::<E, O, PO>::exec_internal(item_spec_graph, &mut resources)
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
            progress_output,
            resources,
            states_type_regs,
        ));
        Ok(cmd_context)
    }
}

impl<E, O, PO> Default for StatesDiscoverCmd<E, O, PO> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
