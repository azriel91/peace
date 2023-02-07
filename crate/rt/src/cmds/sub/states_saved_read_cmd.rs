use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use peace_cfg::{FlowId, ItemSpecId};
use peace_resources::{
    paths::{FlowDir, StatesSavedFile},
    resources::ts::{SetUp, WithStatesSaved},
    states::StatesSaved,
    type_reg::untagged::{BoxDtDisplay, TypeReg},
    Resources,
};
use peace_rt_model::{cmd::CmdContext, Error, StatesSerializer, Storage};
use serde::{de::DeserializeOwned, Serialize};

/// Reads [`StatesSaved`]s from storage.
#[derive(Debug)]
pub struct StatesSavedReadCmd<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>(
    PhantomData<(E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK)>,
);

impl<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
    StatesSavedReadCmd<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
where
    E: std::error::Error + From<Error> + Send,
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    /// Reads [`StatesSaved`]s from storage.
    ///
    /// Either [`StatesCurrentDiscoverCmd`] or [`StatesDiscoverCmd`] must have
    /// run prior to this command to read the state.
    ///
    /// [`StatesCurrentDiscoverCmd`]: crate::StatesCurrentDiscoverCmd
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec(
        mut cmd_context: CmdContext<'_, E, O, SetUp, WorkspaceParamsK, ProfileParamsK, FlowParamsK>,
    ) -> Result<
        CmdContext<'_, E, O, WithStatesSaved, WorkspaceParamsK, ProfileParamsK, FlowParamsK>,
        E,
    > {
        let CmdContext {
            resources,
            states_type_regs,
            ..
        } = &mut cmd_context;

        let states_saved =
            Self::exec_internal(resources, states_type_regs.states_current_type_reg()).await?;

        let cmd_context = CmdContext::from((cmd_context, |resources| {
            Resources::<WithStatesSaved>::from((resources, states_saved))
        }));

        Ok(cmd_context)
    }

    /// Returns [`StatesSaved`] of all [`ItemSpec`]s if it exists on disk.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub(crate) async fn exec_internal(
        resources: &mut Resources<SetUp>,
        states_current_type_reg: &TypeReg<ItemSpecId, BoxDtDisplay>,
    ) -> Result<StatesSaved, E> {
        let flow_id = resources.borrow::<FlowId>();
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_saved_file = StatesSavedFile::from(&*flow_dir);

        let states_saved = StatesSerializer::deserialize_saved(
            &flow_id,
            &storage,
            states_current_type_reg,
            &states_saved_file,
        )
        .await?;

        drop(storage);
        drop(flow_dir);
        drop(flow_id);

        resources.insert(states_saved_file);

        Ok(states_saved)
    }
}

impl<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK> Default
    for StatesSavedReadCmd<E, O, WorkspaceParamsK, ProfileParamsK, FlowParamsK>
{
    fn default() -> Self {
        Self(PhantomData)
    }
}
