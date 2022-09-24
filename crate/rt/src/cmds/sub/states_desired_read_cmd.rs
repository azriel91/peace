use std::marker::PhantomData;

use peace_cfg::ItemSpecId;
use peace_resources::{
    paths::{FlowDir, StatesDesiredFile},
    resources_type_state::{SetUp, WithStatesDesired},
    states::StatesDesired,
    type_reg::untagged::TypeReg,
    Resources,
};
use peace_rt_model::{CmdContext, Error, Storage};

/// Reads [`StatesDesired`]s from storage.
#[derive(Debug)]
pub struct StatesDesiredReadCmd<E, O>(PhantomData<(E, O)>);

impl<E, O> StatesDesiredReadCmd<E, O>
where
    E: std::error::Error + From<Error> + Send,
{
    /// Reads [`StatesDesired`]s from storage.
    ///
    /// Either [`StatesDesiredDiscoverCmd`] or [`StatesDiscoverCmd`] must have
    /// run prior to this command to read the state.
    ///
    /// [`StatesDesiredDiscoverCmd`]: crate::StatesDesiredDiscoverCmd
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec(
        mut cmd_context: CmdContext<'_, E, O, SetUp>,
    ) -> Result<CmdContext<'_, E, O, WithStatesDesired>, E> {
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

    /// Returns the [`StatesDesired`] of all [`ItemSpec`]s if it exists on disk.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub(crate) async fn exec_internal(
        resources: &mut Resources<SetUp>,
        states_desired_type_reg: &TypeReg<ItemSpecId>,
    ) -> Result<StatesDesired, E> {
        let states = Self::deserialize_internal(resources, states_desired_type_reg).await?;

        Ok(states)
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn deserialize_internal(
        resources: &mut Resources<SetUp>,
        states_desired_type_reg: &TypeReg<ItemSpecId>,
    ) -> Result<StatesDesired, E> {
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_desired_file = StatesDesiredFile::from(&*flow_dir);

        if !states_desired_file.exists() {
            return Err(E::from(Error::StatesDesiredDiscoverRequired));
        }

        let states_desired = storage
            .read_with_sync_api(
                "states_desired_file_read".to_string(),
                &states_desired_file,
                |file| {
                    let deserializer = serde_yaml::Deserializer::from_reader(file);
                    let states_desired = StatesDesired::from(
                        states_desired_type_reg
                            .deserialize_map(deserializer)
                            .map_err(Error::StatesDesiredDeserialize)?,
                    );
                    Ok(states_desired)
                },
            )
            .await?;
        drop(flow_dir);
        drop(storage);

        resources.insert(states_desired_file);

        Ok(states_desired)
    }

    #[cfg(target_arch = "wasm32")]
    async fn deserialize_internal(
        resources: &mut Resources<SetUp>,
        states_desired_type_reg: &TypeReg<ItemSpecId>,
    ) -> Result<StatesDesired, E> {
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_desired_file = StatesDesiredFile::from(&*flow_dir);

        let states_desired_file_str = states_desired_file.to_string_lossy();
        let states_serialized = storage
            .get_item(states_desired_file_str.as_ref())?
            .ok_or(Error::StatesDesiredDiscoverRequired)?;
        let deserializer = serde_yaml::Deserializer::from_str(&states_serialized);
        let states_desired = StatesDesired::from(
            states_desired_type_reg
                .deserialize_map(deserializer)
                .map_err(Error::StatesDesiredDeserialize)?,
        );

        drop(flow_dir);
        drop(storage);

        resources.insert(states_desired_file);

        Ok(states_desired)
    }
}
