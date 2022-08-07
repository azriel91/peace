use std::marker::PhantomData;

use futures::{StreamExt, TryStreamExt};
use peace_resources::{
    resources_type_state::{SetUp, WithStateDiffs, WithStatesCurrentAndDesired},
    Resources, StateDiffs, StateDiffsMut,
};
use peace_rt_model::ItemSpecGraph;

use crate::{StateCurrentCmd, StateDesiredCmd};

#[derive(Debug)]
pub struct DiffCmd<E>(PhantomData<E>);

impl<E> DiffCmd<E>
where
    E: std::error::Error,
{
    /// Runs [`StateCurrentFnSpec`]` and `[`StateDesiredFnSpec`]`::`[`exec`] for
    /// each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`States`] and [`StatesDesired`].
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
    /// [`States`]: peace_resources::States
    /// [`StatesRw`]: peace_resources::StatesRw
    /// [`StateCurrentFnSpec`]: peace_cfg::ItemSpec::StateCurrentFnSpec
    /// [`StateDesiredFnSpec`]: peace_cfg::ItemSpec::StateDesiredFnSpec
    pub async fn exec(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: Resources<SetUp>,
    ) -> Result<Resources<WithStateDiffs>, E> {
        let states = StateCurrentCmd::exec_internal(item_spec_graph, &resources).await?;
        let states_desired = StateDesiredCmd::exec_internal(item_spec_graph, &resources).await?;

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

        Ok(Resources::<WithStateDiffs>::from((resources, state_diffs)))
    }
}
