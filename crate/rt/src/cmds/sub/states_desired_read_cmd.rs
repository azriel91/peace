use std::{fmt::Debug, marker::PhantomData};

use peace_cfg::{FlowId, ItemSpecId};
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{
    paths::{FlowDir, StatesDesiredFile},
    resources::ts::SetUp,
    states::StatesDesired,
    type_reg::untagged::{BoxDtDisplay, TypeReg},
    Resources,
};
use peace_rt_model::{params::ParamsKeys, Error, StatesSerializer, Storage};

/// Reads [`StatesDesired`]s from storage.
#[derive(Debug)]
pub struct StatesDesiredReadCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> StatesDesiredReadCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Reads [`StatesDesired`]s from storage.
    ///
    /// [`StatesDiscoverCmd`] must have run prior to this command to read the
    /// state.
    ///
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<StatesDesired, E> {
        let SingleProfileSingleFlowView {
            states_type_reg,
            resources,
            ..
        } = cmd_ctx.scope_mut().view();

        let states_desired = Self::deserialize_internal(resources, states_type_reg).await?;

        Ok(states_desired)
    }

    pub(crate) async fn deserialize_internal(
        resources: &mut Resources<SetUp>,
        states_type_reg: &TypeReg<ItemSpecId, BoxDtDisplay>,
    ) -> Result<StatesDesired, E> {
        let flow_id = resources.borrow::<FlowId>();
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_desired_file = StatesDesiredFile::from(&*flow_dir);

        let states_desired = StatesSerializer::deserialize_desired(
            &flow_id,
            &storage,
            states_type_reg,
            &states_desired_file,
        )
        .await?;

        drop(storage);
        drop(flow_dir);
        drop(flow_id);

        resources.insert(states_desired_file);

        Ok(states_desired)
    }
}

impl<E, O, PKeys> Default for StatesDesiredReadCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
