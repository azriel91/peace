use async_trait::async_trait;
use peace_cmd::scopes::SingleProfileSingleFlowView;
use peace_resources::resources::ts::SetUp;
use peace_rt_model::{outcomes::CmdOutcome, params::ParamsKeys};
use tokio::sync::mpsc::UnboundedSender;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::progress::ProgressUpdateAndId;
        use tokio::sync::mpsc::Sender;
    }
}

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
pub trait CmdBlock {
    /// Automation software error type.
    type Error: std::error::Error + From<peace_rt_model::Error> + Send + 'static;
    /// Types used for params keys.
    type PKeys: ParamsKeys + 'static;
    /// Outcome type of each item's execution, e.g. `ItemDiscoverOutcome<E>`.
    ///
    /// This is usually an enum with variants for the successful and failed
    /// results for each item.
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
    type ItemOutcomeT: Send + 'static;
    /// Outcome type of the command block, e.g. `(StatesCurrent, StatesGoal)`.
    type OutcomeT: 'static;

    /// Producer function to process all items.
    ///
    /// This is infallible because errors are expected to be returned associated
    /// with an item. This may change if there are errors that are related to
    /// the block that are not associated with a specific item.
    async fn exec(
        &self,
        view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        outcome_tx: &UnboundedSender<Self::ItemOutcomeT>,
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
        block_outcome: &mut CmdOutcome<Self::OutcomeT, Self::Error>,
        item_outcome: Self::ItemOutcomeT,
    );
}
