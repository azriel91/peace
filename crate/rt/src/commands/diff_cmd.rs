use std::marker::PhantomData;

use futures::{StreamExt, TryStreamExt};
use peace_resources::{
    internal::StateDiffsMut,
    resources_type_state::{SetUp, WithStateDiffs, WithStatesCurrentAndDesired},
    states::StateDiffs,
    Resources,
};
use peace_rt_model::{CmdContext, Error, OutputWrite};

use crate::{StatesCurrentReadCmd, StatesDesiredReadCmd};

#[derive(Debug)]
pub struct DiffCmd<E, O>(PhantomData<(E, O)>);

impl<E, O> DiffCmd<E, O>
where
    E: std::error::Error + From<Error> + Send,
    O: OutputWrite<E>,
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
        cmd_context: CmdContext<'_, E, O, SetUp>,
    ) -> Result<CmdContext<E, O, WithStateDiffs>, E> {
        let (workspace, item_spec_graph, output, mut resources, states_type_regs) =
            cmd_context.into_inner();

        let states_current = StatesCurrentReadCmd::<E, O>::exec_internal(
            &mut resources,
            states_type_regs.states_current_type_reg(),
        )
        .await?;
        let states_desired = StatesDesiredReadCmd::<E, O>::exec_internal(
            &mut resources,
            states_type_regs.states_desired_type_reg(),
        )
        .await?;

        let resources = Resources::<WithStatesCurrentAndDesired>::from((
            resources,
            states_current,
            states_desired,
        ));
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
        let mut cmd_context = CmdContext::from((
            workspace,
            item_spec_graph,
            output,
            resources,
            states_type_regs,
        ));

        {
            let CmdContext {
                resources, output, ..
            } = &mut cmd_context;
            let state_diffs = resources.borrow::<StateDiffs>();

            output.write_state_diffs(&state_diffs).await?;
        }

        // TODO: output.write_err(error) if any error happens while reading states.

        Ok(cmd_context)
    }
}
