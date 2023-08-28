use std::{fmt::Debug, marker::PhantomData};

use peace_cfg::{FlowId, ItemId};
use peace_cmd::scopes::SingleProfileSingleFlowView;
use peace_cmd_rt::{async_trait, CmdBlock};
use peace_resources::{
    paths::{FlowDir, StatesGoalFile},
    resources::ts::SetUp,
    states::StatesGoalStored,
    type_reg::untagged::{BoxDtDisplay, TypeReg},
    Resources,
};
use peace_rt_model::{outcomes::CmdOutcome, params::ParamsKeys, Error, StatesSerializer, Storage};
use tokio::sync::mpsc::UnboundedSender;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::progress::ProgressUpdateAndId;
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
pub struct StatesGoalReadCmdBlock<E, PKeys>(PhantomData<(E, PKeys)>);

impl<E, PKeys> StatesGoalReadCmdBlock<E, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
{
    pub(crate) async fn deserialize_internal(
        resources: &mut Resources<SetUp>,
        states_type_reg: &TypeReg<ItemId, BoxDtDisplay>,
    ) -> Result<StatesGoalStored, E> {
        let flow_id = resources.borrow::<FlowId>();
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_goal_file = StatesGoalFile::from(&*flow_dir);

        let states_goal_stored = StatesSerializer::deserialize_goal(
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

impl<E, PKeys> Default for StatesGoalReadCmdBlock<E, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[async_trait(?Send)]
impl<E, PKeys> CmdBlock for StatesGoalReadCmdBlock<E, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: Debug + ParamsKeys + 'static,
{
    type Error = E;
    type InputT = ();
    type Outcome = StatesGoalStored;
    type OutcomeAcc = StatesGoalStored;
    type OutcomePartial = Result<StatesGoalStored, E>;
    type PKeys = PKeys;

    fn outcome_acc_init(&self) -> Self::OutcomeAcc {
        StatesGoalStored::new()
    }

    fn outcome_from_acc(&self, outcome_acc: Self::OutcomeAcc) -> Self::Outcome {
        outcome_acc
    }

    async fn exec(
        &self,
        _input: Box<Self::InputT>,
        cmd_view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        outcomes_tx: &UnboundedSender<Self::OutcomePartial>,
        #[cfg(feature = "output_progress")] _progress_tx: &Sender<ProgressUpdateAndId>,
    ) {
        let SingleProfileSingleFlowView {
            states_type_reg,
            resources,
            ..
        } = cmd_view;

        let states_goal_stored = Self::deserialize_internal(resources, states_type_reg).await;

        outcomes_tx
            .send(states_goal_stored)
            .expect("Failed to send `states_goal_stored`.");
    }

    fn outcome_collate(
        &self,
        block_outcome: &mut CmdOutcome<Self::OutcomeAcc, Self::Error>,
        outcome_partial: Self::OutcomePartial,
    ) -> Result<(), Self::Error> {
        let states_goal_stored = outcome_partial?;
        block_outcome.value = states_goal_stored;
        Ok(())
    }
}
