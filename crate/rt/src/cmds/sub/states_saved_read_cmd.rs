use std::marker::PhantomData;

use peace_cfg::ItemSpecId;
use peace_resources::{
    paths::{FlowDir, StatesSavedFile},
    resources::ts::{SetUp, WithStatesSaved},
    states::StatesSaved,
    type_reg::untagged::{BoxDtDisplay, TypeReg},
    Resources,
};
use peace_rt_model::{CmdContext, Error, StatesDeserializer, Storage};

/// Reads [`StatesSaved`]s from storage.
#[derive(Debug)]
pub struct StatesSavedReadCmd<E, O>(PhantomData<(E, O)>);

impl<E, O> StatesSavedReadCmd<E, O>
where
    E: std::error::Error + From<Error> + Send,
{
    /// Reads [`StatesSaved`]s from storage.
    ///
    /// Either [`StatesCurrentDiscoverCmd`] or [`StatesDiscoverCmd`] must have
    /// run prior to this command to read the state.
    ///
    /// [`StatesCurrentDiscoverCmd`]: crate::StatesCurrentDiscoverCmd
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec(
        mut cmd_context: CmdContext<'_, E, O, SetUp>,
    ) -> Result<CmdContext<'_, E, O, WithStatesSaved>, E> {
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
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_saved_file = StatesSavedFile::from(&*flow_dir);

        let states_saved = StatesDeserializer::deserialize_saved(
            &storage,
            states_current_type_reg,
            &states_saved_file,
        )
        .await?;

        drop(flow_dir);
        drop(storage);

        resources.insert(states_saved_file);

        Ok(states_saved)
    }
}

impl<E, O> Default for StatesSavedReadCmd<E, O> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
