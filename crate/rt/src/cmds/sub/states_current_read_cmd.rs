use std::marker::PhantomData;

use peace_cfg::ItemSpecId;
use peace_resources::{
    paths::{FlowDir, StatesCurrentFile},
    resources_type_state::{SetUp, WithStates},
    states::StatesCurrent,
    type_reg::untagged::TypeReg,
    Resources,
};
use peace_rt_model::{CmdContext, Error, Storage};

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
        states_current_type_reg: &TypeReg<ItemSpecId>,
    ) -> Result<StatesCurrent, E> {
        let states = Self::deserialize_internal(resources, states_current_type_reg).await?;

        Ok(states)
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn deserialize_internal(
        resources: &mut Resources<SetUp>,
        states_current_type_reg: &TypeReg<ItemSpecId>,
    ) -> Result<StatesCurrent, E> {
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_current_file = StatesCurrentFile::from(&*flow_dir);

        if !states_current_file.exists() {
            return Err(E::from(Error::StatesCurrentDiscoverRequired));
        }

        let states_current = storage
            .read_with_sync_api(
                "states_current_file_read".to_string(),
                &states_current_file,
                |file| {
                    let deserializer = serde_yaml::Deserializer::from_reader(file);
                    let states_current = StatesCurrent::from(
                        states_current_type_reg
                            .deserialize_map(deserializer)
                            .map_err(Error::StatesCurrentDeserialize)?,
                    );
                    Ok(states_current)
                },
            )
            .await?;
        drop(flow_dir);
        drop(storage);

        resources.insert(states_current_file);

        Ok(states_current)
    }

    #[cfg(target_arch = "wasm32")]
    async fn deserialize_internal(
        resources: &mut Resources<SetUp>,
        states_current_type_reg: &TypeReg<ItemSpecId>,
    ) -> Result<StatesCurrent, E> {
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_current_file = StatesCurrentFile::from(&*flow_dir);

        let states_current_file_str = states_current_file.to_string_lossy();
        let states_serialized = storage
            .get_item_opt(states_current_file_str.as_ref())?
            .ok_or(Error::StatesCurrentDiscoverRequired)?;
        let deserializer = serde_yaml::Deserializer::from_str(&states_serialized);
        let states_current = StatesCurrent::from(
            states_current_type_reg
                .deserialize_map(deserializer)
                .map_err(Error::StatesCurrentDeserialize)?,
        );

        drop(flow_dir);
        drop(storage);

        resources.insert(states_current_file);

        Ok(states_current)
    }
}
