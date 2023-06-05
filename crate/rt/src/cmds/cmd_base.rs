use std::{fmt::Debug, future::Future, marker::PhantomData};

use peace_cfg::ItemId;
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_params::ParamsSpecs;
use peace_resources::{resources::ts::SetUp, Resources};
use peace_rt_model::{
    outcomes::CmdOutcome, output::OutputWrite, params::ParamsKeys, Error, Flow, IndexMap,
};
use tokio::sync::{mpsc, mpsc::UnboundedSender};

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::progress::ProgressUpdateAndId;
        use peace_rt_model::CmdProgressTracker;
        use tokio::sync::mpsc::Sender;
    }
}

/// Common code to run the item execution, progress rendering, and item
/// outcome collection async tasks.
#[derive(Debug)]
pub struct CmdBase<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> CmdBase<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
    O: OutputWrite<E>,
{
    /// Common code to run the item execution, progress rendering, and item
    /// outcome collection async tasks.
    ///
    /// # Type Parameters
    ///
    /// * `ItemOutcomeT`: Outcome type for each item's execution.
    /// * `FnExecFut`: Future returned by `FnExec`, which resolves to
    ///   `ItemOutcomeT`.
    /// * `FnExec`: Function for the `Cmd` implementation to execute its work.
    ///
    ///     This is the producer function to process all items.
    ///
    ///     This is infallible because errors are expected to be returned
    ///     associated with an item. This may change if there are errors
    ///     that are related to the framework that cannot be associated with
    ///     an item.
    ///
    /// * `OutcomeT`: Outcome type for the overall command execution.
    /// * `FnOutcomeCollate`: Function for the `Cmd` to collect the outcome of
    ///   each item.
    ///
    ///     This is the consumer function to collect the outcome of each item,
    ///     whether successful or not.
    ///
    ///     This is not async because at the time of writing, this is expected
    ///     to write into an in-memory map. This may change in the future if
    ///     there is work that could benefit from being asynchronous.
    ///
    ///     This is infallible because errors are expected to be collected and
    ///     associated with an item. This may change if there are errors that
    ///     are related to the framework that cannot be associated with an
    ///     item.
    ///
    /// [`exec`]: peace_cfg::ApplyFns::exec
    /// [`Item`]: peace_cfg::Item
    /// [`ApplyFns`]: peace_cfg::Item::ApplyFns
    pub async fn exec<
        'f,
        ItemOutcomeT,
        FnExecFut,
        // This abomination is because Rust supports attributes on type parameters in the function
        // signature, but not on inner type parameters, or `TypeParamBounds`
        //
        // See <https://doc.rust-lang.org/reference/items/functions.html> and related grammar.
        #[cfg(feature = "output_progress")] FnExec: FnOnce(
                &Flow<E>,
                &ParamsSpecs,
                &Resources<SetUp>,
                &Sender<ProgressUpdateAndId>,
                &UnboundedSender<ItemOutcomeT>,
            ) -> FnExecFut
            + 'f,
        #[cfg(not(feature = "output_progress"))] FnExec: FnOnce(
                &Flow<E>,
                &ParamsSpecs,
                &Resources<SetUp>,
                &UnboundedSender<ItemOutcomeT>,
            ) -> FnExecFut
            + 'f,
        OutcomeT,
        FnOutcomeCollate,
    >(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        mut outcome: OutcomeT,
        fn_exec: FnExec,
        fn_outcome: FnOutcomeCollate,
    ) -> Result<CmdOutcome<OutcomeT, E>, E>
    where
        ItemOutcomeT: Send + 'static,
        FnExecFut: Future<Output = ItemOutcomeT> + 'f,
        OutcomeT: Send + 'static,
        FnOutcomeCollate: Fn(&mut OutcomeT, &mut IndexMap<ItemId, E>, ItemOutcomeT),
    {
        let SingleProfileSingleFlowView {
            #[cfg(feature = "output_progress")]
            output,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            flow,
            params_specs,
            resources,
            ..
        } = cmd_ctx.view();

        cfg_if::cfg_if! {
            if #[cfg(feature = "output_progress")] {
                output.progress_begin(cmd_progress_tracker).await;

                let CmdProgressTracker {
                    multi_progress: _,
                    progress_trackers,
                    ..
                } = cmd_progress_tracker;

                let (progress_tx, progress_rx) =
                    mpsc::channel::<ProgressUpdateAndId>(crate::PROGRESS_COUNT_MAX);
            }
        }

        let (outcomes_tx, mut outcomes_rx) = mpsc::unbounded_channel::<ItemOutcomeT>();

        let resources_ref = &*resources;
        let execution_task = async move {
            #[cfg(feature = "output_progress")]
            let progress_tx = &progress_tx;
            let outcomes_tx = &outcomes_tx;

            fn_exec(
                flow,
                params_specs,
                resources_ref,
                #[cfg(feature = "output_progress")]
                progress_tx,
                outcomes_tx,
            )
            .await;

            // `progress_tx` is dropped here, so `progress_rx` will safely end.
        };

        #[cfg(feature = "output_progress")]
        let progress_render_task =
            crate::progress::Progress::progress_render(output, progress_trackers, progress_rx);

        let mut errors = IndexMap::<ItemId, E>::new();
        let outcomes_rx_task = async {
            while let Some(item_outcome) = outcomes_rx.recv().await {
                fn_outcome(&mut outcome, &mut errors, item_outcome);
            }
        };

        cfg_if::cfg_if! {
            if #[cfg(feature = "output_progress")] {
                futures::join!(execution_task, progress_render_task, outcomes_rx_task);

                output.progress_end(cmd_progress_tracker).await;
            } else {
                futures::join!(execution_task, outcomes_rx_task);
            }
        }

        Ok(CmdOutcome {
            value: outcome,
            errors,
        })
    }
}

impl<E, O, PKeys> Default for CmdBase<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
