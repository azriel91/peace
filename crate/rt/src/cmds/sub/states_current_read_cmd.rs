use std::marker::PhantomData;

use peace_cfg::ItemSpecId;
use peace_resources::{
    paths::{FlowDir, StatesCurrentFile},
    resources::ts::{SetUp, WithStates},
    states::StatesCurrent,
    type_reg::untagged::{BoxDtDisplay, TypeReg},
    Resources,
};
use peace_rt_model::{CmdContext, Error, StatesDeserializer, Storage};

/// Reads [`StatesCurrent`]s from storage.
#[derive(Debug)]
pub struct StatesCurrentReadCmd<E, O>(PhantomData<(E, O)>);

impl<E, O> StatesCurrentReadCmd<E, O>
where
    E: std::error::Error + From<Error> + Send,
{
    /// Reads [`StatesCurrent`]s from storage.
    ///
    /// Either [`StatesCurrentDiscoverCmd`] or [`StatesDiscoverCmd`] must have
    /// run prior to this command to read the state.
    ///
    /// [`StatesCurrentDiscoverCmd`]: crate::StatesCurrentDiscoverCmd
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec(
        mut cmd_context: CmdContext<'_, E, O, SetUp>,
    ) -> Result<CmdContext<'_, E, O, WithStates>, E> {
        let CmdContext {
            resources,
            states_type_regs,
            ..
        } = &mut cmd_context;

        let states_current =
            Self::exec_internal(resources, states_type_regs.states_current_type_reg()).await?;

        let cmd_context = CmdContext::from((cmd_context, |resources| {
            Resources::<WithStates>::from((resources, states_current))
        }));

        Ok(cmd_context)
    }

    /// Returns the [`StatesCurrent`] of all [`ItemSpec`]s if it exists on disk.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub(crate) async fn exec_internal(
        resources: &mut Resources<SetUp>,
        states_current_type_reg: &TypeReg<ItemSpecId, BoxDtDisplay>,
    ) -> Result<StatesCurrent, E> {
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_current_file = StatesCurrentFile::from(&*flow_dir);

        let states_current = StatesDeserializer::deserialize(
            &storage,
            states_current_type_reg,
            &states_current_file,
        )
        .await?;

        drop(flow_dir);
        drop(storage);

        resources.insert(states_current_file);

        Ok(states_current)
    }
}

impl<E, O> Default for StatesCurrentReadCmd<E, O> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
