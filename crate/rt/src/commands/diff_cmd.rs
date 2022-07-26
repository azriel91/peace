use std::marker::PhantomData;

use peace_resources::{
    resources_type_state::{SetUp, WithStatesNowAndDesired},
    Resources,
};
use peace_rt_model::FullSpecGraph;

use crate::{StateDesiredCmd, StateNowCmd};

#[derive(Debug)]
pub struct DiffCmd<E>(PhantomData<E>);

impl<E> DiffCmd<E>
where
    E: std::error::Error,
{
    /// Runs [`StateNowFnSpec`]` and `[`StateDesiredFnSpec`]`::`[`exec`] for
    /// each [`FullSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`States`] and [`StatesDesired`].
    ///
    /// If any `StateNowFnSpec` needs to read the `State` from a previous
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
    /// [`StateNowFnSpec`]: peace_cfg::FullSpec::StateNowFnSpec
    /// [`StateDesiredFnSpec`]: peace_cfg::FullSpec::StateDesiredFnSpec
    pub async fn exec(
        full_spec_graph: &FullSpecGraph<E>,
        resources: Resources<SetUp>,
    ) -> Result<Resources<WithStatesNowAndDesired>, E> {
        StateNowCmd::exec_internal(full_spec_graph, &resources).await?;
        StateDesiredCmd::exec_internal(full_spec_graph, &resources).await?;

        Ok(Resources::<WithStatesNowAndDesired>::from(resources))
    }
}
