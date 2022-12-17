use std::marker::PhantomData;

use peace_cfg::ItemSpecId;
use peace_resources::{
    paths::{FlowDir, StatesPreviousFile},
    resources::ts::{SetUp, WithStatesPrevious},
    states::StatesPrevious,
    type_reg::untagged::{BoxDtDisplay, TypeReg},
    Resources,
};
use peace_rt_model::{CmdContext, Error, StatesDeserializer, Storage};

/// Reads [`StatesPrevious`]s from storage.
#[derive(Debug)]
pub struct StatesPreviousReadCmd<E, O>(PhantomData<(E, O)>);

impl<E, O> StatesPreviousReadCmd<E, O>
where
    E: std::error::Error + From<Error> + Send,
{
    /// Reads [`StatesPrevious`]s from storage.
    ///
    /// Either [`StatesCurrentDiscoverCmd`] or [`StatesDiscoverCmd`] must have
    /// run prior to this command to read the state.
    ///
    /// [`StatesCurrentDiscoverCmd`]: crate::StatesCurrentDiscoverCmd
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec(
        mut cmd_context: CmdContext<'_, E, O, SetUp>,
    ) -> Result<CmdContext<'_, E, O, WithStatesPrevious>, E> {
        let CmdContext {
            resources,
            states_type_regs,
            ..
        } = &mut cmd_context;

        let states_previous =
            Self::exec_internal(resources, states_type_regs.states_current_type_reg()).await?;

        let cmd_context = CmdContext::from((cmd_context, |resources| {
            Resources::<WithStatesPrevious>::from((resources, states_previous))
        }));

        Ok(cmd_context)
    }

    /// Returns [`StatesPrevious`] of all [`ItemSpec`]s if it exists on disk.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub(crate) async fn exec_internal(
        resources: &mut Resources<SetUp>,
        states_current_type_reg: &TypeReg<ItemSpecId, BoxDtDisplay>,
    ) -> Result<StatesPrevious, E> {
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_previous_file = StatesPreviousFile::from(&*flow_dir);

        let states_previous = StatesDeserializer::deserialize_previous(
            &storage,
            states_current_type_reg,
            &states_previous_file,
        )
        .await?;

        drop(flow_dir);
        drop(storage);

        resources.insert(states_previous_file);

        Ok(states_previous)
    }
}

impl<E, O> Default for StatesPreviousReadCmd<E, O> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
