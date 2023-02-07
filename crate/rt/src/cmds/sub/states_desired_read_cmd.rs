use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use peace_cfg::ItemSpecId;
use peace_resources::{
    paths::{FlowDir, StatesDesiredFile},
    resources::ts::{SetUp, WithStatesDesired},
    states::StatesDesired,
    type_reg::untagged::{BoxDtDisplay, TypeReg},
    Resources,
};
use peace_rt_model::{cmd::CmdContext, Error, StatesSerializer, Storage};
use serde::{de::DeserializeOwned, Serialize};

/// Reads [`StatesDesired`]s from storage.
#[derive(Debug)]
pub struct StatesDesiredReadCmd<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>(
    PhantomData<(E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK)>,
);

impl<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
    StatesDesiredReadCmd<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
where
    E: std::error::Error + From<Error> + Send,
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    /// Reads [`StatesDesired`]s from storage.
    ///
    /// Either [`StatesDesiredDiscoverCmd`] or [`StatesDiscoverCmd`] must have
    /// run prior to this command to read the state.
    ///
    /// [`StatesDesiredDiscoverCmd`]: crate::StatesDesiredDiscoverCmd
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec(
        mut cmd_context: CmdContext<'_, E, O, SetUp, WorkspaceParamsK, ProfileParamsK, FlowParamsK>,
    ) -> Result<
        CmdContext<'_, E, O, WithStatesDesired, WorkspaceParamsK, ProfileParamsK, FlowParamsK>,
        E,
    > {
        let CmdContext {
            resources,
            states_type_regs,
            ..
        } = &mut cmd_context;

        let states_desired =
            Self::exec_internal(resources, states_type_regs.states_desired_type_reg()).await?;

        let cmd_context = CmdContext::from((cmd_context, |resources| {
            Resources::<WithStatesDesired>::from((resources, states_desired))
        }));

        Ok(cmd_context)
    }

    /// Returns [`StatesDesired`] of all [`ItemSpec`]s if it exists on disk.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub(crate) async fn exec_internal(
        resources: &mut Resources<SetUp>,
        states_desired_type_reg: &TypeReg<ItemSpecId, BoxDtDisplay>,
    ) -> Result<StatesDesired, E> {
        let states = Self::deserialize_internal(resources, states_desired_type_reg).await?;

        Ok(states)
    }

    async fn deserialize_internal(
        resources: &mut Resources<SetUp>,
        states_desired_type_reg: &TypeReg<ItemSpecId, BoxDtDisplay>,
    ) -> Result<StatesDesired, E> {
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_desired_file = StatesDesiredFile::from(&*flow_dir);

        let states_desired = StatesSerializer::deserialize_desired(
            &storage,
            states_desired_type_reg,
            &states_desired_file,
        )
        .await?;

        drop(flow_dir);
        drop(storage);

        resources.insert(states_desired_file);

        Ok(states_desired)
    }
}

impl<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK> Default
    for StatesDesiredReadCmd<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
{
    fn default() -> Self {
        Self(PhantomData)
    }
}
