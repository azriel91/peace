use std::fmt::Debug;

use async_trait::async_trait;
use peace_cmd::scopes::SingleProfileSingleFlowView;
use peace_resources::{resources::ts::SetUp, Resource, Resources};
use peace_rt_model::{outcomes::CmdOutcome, params::ParamsKeys};
use tokio::sync::mpsc::UnboundedSender;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::progress::ProgressUpdateAndId;
        use tokio::sync::mpsc::Sender;
    }
}

pub use self::{
    cmd_block_error::CmdBlockError, cmd_block_rt::CmdBlockRt, cmd_block_rt_box::CmdBlockRtBox,
    cmd_block_wrapper::CmdBlockWrapper,
};

mod cmd_block_error;
mod cmd_block_rt;
mod cmd_block_rt_box;
mod cmd_block_wrapper;

/// Runs one [`Item::*`] function for one iteration of items.
///
/// A command may consist of:
///
/// 1. Discovering the current state of an environment.
/// 2. Ensuring new items that are not blocked, e.g. launch new servers before
///    taking old servers away.
/// 3. Cleaning unused items that block new items from being ensured, e.g.
///    terminating servers before resizing a subnet's CIDR block.
/// 4. Ensuring new / modified items that are newly unblocked, e.g. launching
///    new servers in the resized subnet.
/// 5. Cleaning unused items that are no longer needed, e.g. removing an old
///    service.
///
/// Each of these is an iteration through items, running one of the [`Item::*`]
/// functions.
///
/// A `CmdBlock` is the unit of one iteration logic.
///
/// [`Item::*`]: peace_cfg::Item
#[async_trait(?Send)]
pub trait CmdBlock: Debug {
    /// Automation software error type.
    type Error: std::error::Error + From<peace_rt_model::Error> + Send + 'static;
    /// Types used for params keys.
    type PKeys: ParamsKeys + 'static;
    /// Outcome type of the command block, e.g. `(StatesCurrent, StatesGoal)`.
    type Outcome: Debug + Send + Sync + 'static;
    /// Intermediate working type of the command block, e.g.
    /// `StatesMut<Ensured>`.
    type OutcomeAcc: Resource + 'static;
    /// Type to represent information collected during execution, e.g.
    /// `ItemDiscoverOutcome<E>`.
    ///
    /// This can be:
    ///
    /// * the initialization of the block outcome.
    /// * the result of running an item's `apply` function.
    ///
    /// An example of this is an enum with variants for the successful and
    /// failed results for each item.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// #[derive(Debug)]
    /// enum ItemDiscoverOutcome<E> {
    ///     /// Discover succeeded.
    ///     Success {
    ///         item_id: ItemId,
    ///         state_current: Option<BoxDtDisplay>,
    ///         state_goal: Option<BoxDtDisplay>,
    ///     },
    ///     /// Discover failed.
    ///     Fail {
    ///         item_id: ItemId,
    ///         state_current: Option<BoxDtDisplay>,
    ///         state_goal: Option<BoxDtDisplay>,
    ///         error: E,
    ///     },
    /// }
    /// ```
    type OutcomePartial: Send + 'static;
    /// Input type of the command block, e.g. `StatesCurrent`.
    type InputT: Resource + 'static;

    /// Fetch function for `InputT`.
    ///
    /// This is overridable so that `CmdBlock`s can change how their `InputT` is
    /// looked up.
    ///
    /// The most common use case for overriding this is for unit `()` inputs,
    /// which should provide an empty implementation.
    fn input_fetch(&self, resources: &mut Resources<SetUp>) -> Self::InputT {
        resources.remove::<Self::InputT>().unwrap_or_else(|| {
            let input_type_name = tynm::type_name::<Self::InputT>();
            panic!(
                "Expected `{input_type_name}` to exist in `Resources`.\n\
                Make sure a previous `CmdBlock` has that type as its `Outcome`."
            );
        })
    }

    /// Seed function for `OutcomeAcc`.
    fn outcome_acc_init(&self, input: &Self::InputT) -> Self::OutcomeAcc;

    /// Maps the `outcome_acc` into `outcome`.
    fn outcome_from_acc(&self, outcome_acc: Self::OutcomeAcc) -> Self::Outcome;

    /// Inserts the `CmdBlock::Outcome` into `Resources`.
    ///
    /// This is overridable so that `CmdBlock`s can change how their `Outcome`
    /// is inserted.
    ///
    /// The most common use case for overriding this is for unit `()` inputs,
    /// which should provide an empty implementation.
    fn outcome_insert(&self, resources: &mut Resources<SetUp>, outcome: Self::Outcome) {
        resources.insert(outcome);
    }

    /// Producer function to process all items.
    ///
    /// This is infallible because errors are expected to be returned associated
    /// with an item. This may change if there are errors that are related to
    /// the block that are not associated with a specific item.
    async fn exec(
        &self,
        input: Self::InputT,
        cmd_view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        outcomes_tx: &UnboundedSender<Self::OutcomePartial>,
        #[cfg(feature = "output_progress")] progress_tx: &Sender<ProgressUpdateAndId>,
    );

    /// Collects item outcomes into a command outcome.
    ///
    /// This is not async because at the time of writing, this is expected to
    /// write into an in-memory map. This may change in the future if there is
    /// work that could benefit from being asynchronous.
    ///
    /// This is infallible because errors are expected to be collected and
    /// associated with an item. This may change if there are errors that are
    /// related to the framework that cannot be associated with an item.
    fn outcome_collate(
        &self,
        block_outcome: &mut CmdOutcome<Self::OutcomeAcc, Self::Error>,
        outcome_partial: Self::OutcomePartial,
    ) -> Result<(), Self::Error>;
}
