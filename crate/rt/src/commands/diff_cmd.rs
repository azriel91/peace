use std::marker::PhantomData;

use futures::{StreamExt, TryStreamExt};
use peace_resources::{
    resources_type_state::{SetUp, WithStateDiffs, WithStatesCurrentAndDesired},
    Resources, StateDiffs, StateDiffsMut,
};
use peace_rt_model::FullSpecGraph;

use crate::{StateCurrentCmd, StateDesiredCmd};

#[derive(Debug)]
pub struct DiffCmd<E>(PhantomData<E>);

impl<E> DiffCmd<E>
where
    E: std::error::Error,
{
    /// Runs [`StateCurrentFnSpec`]` and `[`StateDesiredFnSpec`]`::`[`exec`] for
    /// each [`FullSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`States`] and [`StatesDesired`].
    ///
    /// If any `StateCurrentFnSpec` needs to read the `State` from a previous
    /// `FullSpec`, the [`StatesRw`] type should be used in
    /// [`FnSpec::Data`].
    ///
    /// Likewise, if any `StateDesiredFnSpec` needs to read the desired `State`
    /// from a previous `FullSpec`, the [`StatesDesiredRw`] type should be
    /// used in [`FnSpec::Data`].
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`FnSpec::Data`]: peace_cfg::FnSpec::Data
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`States`]: peace_resources::States
    /// [`StatesRw`]: peace_resources::StatesRw
    /// [`StateCurrentFnSpec`]: peace_cfg::FullSpec::StateCurrentFnSpec
    /// [`StateDesiredFnSpec`]: peace_cfg::FullSpec::StateDesiredFnSpec
    pub async fn exec(
        full_spec_graph: &FullSpecGraph<E>,
        resources: Resources<SetUp>,
    ) -> Result<Resources<WithStateDiffs>, E> {
        let states = StateCurrentCmd::exec_internal(full_spec_graph, &resources).await?;
        let states_desired = StateDesiredCmd::exec_internal(full_spec_graph, &resources).await?;

        let resources =
            Resources::<WithStatesCurrentAndDesired>::from((resources, states, states_desired));
        let resources_ref = &resources;
        let state_diffs = {
            let state_diffs_mut = full_spec_graph
                .stream()
                .map(Result::<_, E>::Ok)
                .and_then(|full_spec| async move {
                    Ok((
                        full_spec.id(),
                        full_spec.state_diff_fn_exec(resources_ref).await?,
                    ))
                })
                .try_collect::<StateDiffsMut>()
                .await?;

            StateDiffs::from(state_diffs_mut)
        };

        Ok(Resources::<WithStateDiffs>::from((resources, state_diffs)))
    }
}
