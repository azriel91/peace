use std::{fmt::Debug, marker::PhantomData};

use peace_cmd_ctx::{CmdCtxSpsfFields, CmdCtxTypes};
use peace_cmd_model::CmdBlockOutcome;
use peace_cmd_rt::{async_trait, CmdBlock};
use peace_flow_model::FlowId;
use peace_item_model::ItemId;
use peace_resource_rt::{
    paths::{FlowDir, StatesGoalFile},
    resources::ts::SetUp,
    states::StatesGoalStored,
    type_reg::untagged::{BoxDtDisplay, TypeReg},
    ResourceFetchError, Resources,
};
use peace_rt_model::Storage;
use peace_state_rt::StatesSerializer;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_progress_model::{CmdBlockItemInteractionType, CmdProgressUpdate};
        use tokio::sync::mpsc::Sender;
    }
}

/// Reads [`StatesGoalStored`]s from storage.
///
/// Either [`StatesDiscoverCmdBlock::goal`] or
/// [`StatesDiscoverCmdBlock::current_and_goal`] must have run prior to this
/// command to read the state.
///
/// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
#[derive(Debug)]
pub struct StatesGoalReadCmdBlock<CmdCtxTypesT>(PhantomData<CmdCtxTypesT>);

impl<CmdCtxTypesT> StatesGoalReadCmdBlock<CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    /// Returns a new `StatesGoalReadCmdBlock`.
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) async fn deserialize_internal(
        resources: &mut Resources<SetUp>,
        states_type_reg: &TypeReg<ItemId, BoxDtDisplay>,
    ) -> Result<StatesGoalStored, <CmdCtxTypesT as CmdCtxTypes>::AppError> {
        let flow_id = resources.borrow::<FlowId>();
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_goal_file = StatesGoalFile::from(&*flow_dir);

        let states_goal_stored =
            StatesSerializer::<<CmdCtxTypesT as CmdCtxTypes>::AppError>::deserialize_goal(
                &flow_id,
                &storage,
                states_type_reg,
                &states_goal_file,
            )
            .await?;

        drop(storage);
        drop(flow_dir);
        drop(flow_id);

        resources.insert(states_goal_file);

        Ok(states_goal_stored)
    }
}

impl<CmdCtxTypesT> Default for StatesGoalReadCmdBlock<CmdCtxTypesT> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[async_trait(?Send)]
impl<CmdCtxTypesT> CmdBlock for StatesGoalReadCmdBlock<CmdCtxTypesT>
where
    CmdCtxTypesT: CmdCtxTypes,
{
    type CmdCtxTypes = CmdCtxTypesT;
    type InputT = ();
    type Outcome = StatesGoalStored;

    #[cfg(feature = "output_progress")]
    fn cmd_block_item_interaction_type(&self) -> CmdBlockItemInteractionType {
        CmdBlockItemInteractionType::Local
    }

    fn input_fetch(&self, _resources: &mut Resources<SetUp>) -> Result<(), ResourceFetchError> {
        Ok(())
    }

    fn input_type_names(&self) -> Vec<String> {
        vec![]
    }

    async fn exec(
        &self,
        _input: Self::InputT,
        cmd_ctx_spsf_fields: &mut CmdCtxSpsfFields<'_, Self::CmdCtxTypes>,
        #[cfg(feature = "output_progress")] _progress_tx: &Sender<CmdProgressUpdate>,
    ) -> Result<
        CmdBlockOutcome<Self::Outcome, <Self::CmdCtxTypes as CmdCtxTypes>::AppError>,
        <Self::CmdCtxTypes as CmdCtxTypes>::AppError,
    > {
        let CmdCtxSpsfFields {
            states_type_reg,
            resources,
            ..
        } = cmd_ctx_spsf_fields;

        Self::deserialize_internal(resources, states_type_reg)
            .await
            .map(CmdBlockOutcome::Single)
    }
}
