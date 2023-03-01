use std::{fmt::Debug, marker::PhantomData};

use peace_cfg::{FlowId, ItemSpecId};
use peace_resources::{
    paths::{FlowDir, StatesDesiredFile},
    resources::ts::{SetUp, WithStatesDesired},
    states::StatesDesired,
    type_reg::untagged::{BoxDtDisplay, TypeReg},
    Resources,
};
use peace_rt_model::{
    cmd::CmdContext, cmd_context_params::ParamsKeys, Error, StatesSerializer, Storage,
};

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
    /// Either [`StatesDesiredDiscoverCmd`] or [`StatesDiscoverCmd`] must have
    /// run prior to this command to read the state.
    ///
    /// [`StatesDesiredDiscoverCmd`]: crate::StatesDesiredDiscoverCmd
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec(
        mut cmd_context: CmdContext<'_, E, O, SetUp, PKeys>,
    ) -> Result<CmdContext<'_, E, O, WithStatesDesired, PKeys>, E> {
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
        let flow_id = resources.borrow::<FlowId>();
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_desired_file = StatesDesiredFile::from(&*flow_dir);

        let states_desired = StatesSerializer::deserialize_desired(
            &flow_id,
            &storage,
            states_desired_type_reg,
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
