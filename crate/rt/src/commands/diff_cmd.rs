use std::marker::PhantomData;

use futures::{StreamExt, TryStreamExt};
use peace_resources::{
    internal::StateDiffsMut,
    resources_type_state::{SetUp, WithStateDiffs, WithStatesCurrentAndDesired},
    states::StateDiffs,
    Resources,
};
use peace_rt_model::{CmdContext, Error};

use crate::{StatesCurrentDiscoverCmd, StatesDesiredDiscoverCmd};

#[derive(Debug)]
pub struct DiffCmd<E>(PhantomData<E>);

impl<E> DiffCmd<E>
where
    E: std::error::Error + From<Error> + Send,
{
    /// Runs [`StateCurrentFnSpec`]` and `[`StateDesiredFnSpec`]`::`[`exec`] for
    /// each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCurrent`] and [`StatesDesired`].
    ///
    /// If any `StateCurrentFnSpec` needs to read the `State` from a previous
    /// `ItemSpec`, the [`StatesRw`] type should be used in
    /// [`FnSpec::Data`].
    ///
    /// Likewise, if any `StateDesiredFnSpec` needs to read the desired `State`
    /// from a previous `ItemSpec`, the [`StatesDesiredRw`] type should be
    /// used in [`FnSpec::Data`].
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`FnSpec::Data`]: peace_cfg::FnSpec::Data
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`StatesCurrent`]: peace_resources::StatesCurrent
    /// [`StatesRw`]: peace_resources::StatesRw
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    pub async fn exec(
        cmd_context: CmdContext<'_, SetUp, E>,
    ) -> Result<CmdContext<WithStateDiffs, E>, E> {
        let (workspace, item_spec_graph, mut resources, states_type_regs) =
            cmd_context.into_inner();
        let states =
            StatesCurrentDiscoverCmd::exec_internal(item_spec_graph, &mut resources).await?;
        let states_desired =
            StatesDesiredDiscoverCmd::exec_internal(item_spec_graph, &mut resources).await?;

        let resources =
            Resources::<WithStatesCurrentAndDesired>::from((resources, states, states_desired));
        let resources_ref = &resources;
        let state_diffs = {
            let state_diffs_mut = item_spec_graph
                .stream()
                .map(Result::<_, E>::Ok)
                .and_then(|item_spec| async move {
                    Ok((
                        item_spec.id(),
                        item_spec.state_diff_fn_exec(resources_ref).await?,
                    ))
                })
                .try_collect::<StateDiffsMut>()
                .await?;

            StateDiffs::from(state_diffs_mut)
        };

        let resources = Resources::<WithStateDiffs>::from((resources, state_diffs));
        let cmd_context =
            CmdContext::from((workspace, item_spec_graph, resources, states_type_regs));
        Ok(cmd_context)
    }
}
