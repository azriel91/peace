use std::{fmt::Debug, marker::PhantomData};

use peace_cfg::{FlowId, ItemSpecId};
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{
    paths::{FlowDir, StatesSavedFile},
    resources::ts::SetUp,
    states::StatesSaved,
    type_reg::untagged::{BoxDtDisplay, TypeReg},
    Resources,
};
use peace_rt_model::{params::ParamsKeys, Error, StatesSerializer, Storage};

/// Reads [`StatesSaved`]s from storage.
#[derive(Debug)]
pub struct StatesSavedReadCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> StatesSavedReadCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Reads [`StatesSaved`]s from storage.
    ///
    /// Either [`StatesSavedDiscoverCmd`] or [`StatesDiscoverCmd`] must have
    /// run prior to this command to read the state.
    ///
    /// [`StatesSavedDiscoverCmd`]: crate::StatesSavedDiscoverCmd
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<StatesSaved, E> {
        let SingleProfileSingleFlowView {
            states_type_regs,
            resources,
            ..
        } = cmd_ctx.scope_mut().view();

        let states_saved =
            Self::deserialize_internal(resources, states_type_regs.states_current_type_reg())
                .await?;

        Ok(states_saved)
    }

    pub(crate) async fn deserialize_internal(
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

impl<E, O, PKeys> Default for StatesSavedReadCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
