use std::{fmt::Debug, marker::PhantomData};

use futures::future::LocalBoxFuture;
use peace_cfg::ItemId;
use peace_cmd::{scopes::SingleProfileSingleFlowView, CmdIndependence};
use peace_resources::resources::ts::SetUp;
use peace_rt_model::{
    outcomes::CmdOutcome, output::OutputWrite, params::ParamsKeys, Error, IndexMap,
};
use tokio::sync::{mpsc, mpsc::UnboundedSender};

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cmd::scopes::SingleProfileSingleFlowViewAndOutput;
        use peace_cfg::progress::ProgressUpdateAndId;
        use tokio::sync::mpsc::Sender;
    }
}

/// Common code to handle command composition and progress rendering.
///
/// * For commands that do one-off tasks, see [`CmdBase::oneshot`].
/// * For commands that process items according to a flow's graph, and should
///   render progress, see [`CmdBase::exec`].
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
    pub async fn oneshot<FnExec, OutcomeT>(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
        fn_exec: FnExec,
    ) -> Result<OutcomeT, E>
    where
        FnExec: for<'f_exec> FnOnce(
            &'f_exec mut SingleProfileSingleFlowView<'_, E, PKeys, SetUp>,
        ) -> LocalBoxFuture<'f_exec, Result<OutcomeT, E>>,
    {
        match cmd_independence {
            CmdIndependence::Standalone { cmd_ctx } => fn_exec(&mut cmd_ctx.view()).await,
            CmdIndependence::SubCmd { cmd_view } => fn_exec(cmd_view).await,
            #[cfg(feature = "output_progress")]
            CmdIndependence::SubCmdWithProgress {
                cmd_view,
                progress_tx: _,
            } => fn_exec(cmd_view).await,
        }
    }

    /// Common code to run the item execution, progress rendering, and item
    /// outcome collection async tasks.
    ///
    /// # Type Parameters
    ///
    /// * `ItemOutcomeT`: Outcome type for each item's execution.
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
    /// # Design
    ///
    /// > The technique described here was shared by Yandros (thank you!).
    /// >
    /// > See:
    /// >
    /// > * <https://users.rust-lang.org/t/argument-requires-that-is-borrowed-for-static/66503/2>
    /// > * <https://discord.com/channels/273534239310479360/616791170340880404/1115592255814783006>
    ///
    /// For the default `for<'f> Fn(..)` syntax, the compiler will:
    ///
    /// * Choose the smallest possible lifetime to reduce the parameter lifetime
    ///   requirements (good for callee).
    /// * Choose the largest possible lifetime within that range to support more
    ///   parameter types (bad for caller!).
    ///
    /// For the [`CmdIndependence`] parameter to `FnExec`, there should be an
    /// upper bound of `'fn_exec` to the lifetime of `CmdBase::exec`. However,
    /// at the time of writing (Rust 1.70.0) there is no way to express this
    /// constraint explicitly, i.e. we cannot write:
    ///
    /// ```rust,ignore
    /// FnExec: for<'f_exec where 'f_outer: 'f_exec>` FnOnce(..)
    /// ```
    ///
    /// However, we could use *implicit bounds* to introduce that constraint in
    /// the closure by adding another parameter:
    ///
    /// ```rust,ignore
    /// FnExec(.., [&'fn_exec &'fn_outer (); 0])
    /// ```
    ///
    /// This expresses the constraint that lifts the second behaviour of the
    /// compiler, similar to `PhantomData`.
    ///
    /// However, it requires callers to pass in an additional parameter in the
    /// closure. For `CmdIndependence`, we can choose to:
    ///
    /// * Use separate lifetime parameters for `Scope` and `View`, avoiding the
    ///   `'static` propagation from the `view` to the `O` type parameter.
    /// * Add this additional parameter to the closure.
    ///
    /// I've gone with the first option, to avoid the additional parameter in
    /// the closure.
    pub async fn exec<
        ItemOutcomeT,
        // This abomination is because Rust supports attributes on type parameters in the function
        // signature, but not on inner type parameters, or `TypeParamBounds`
        //
        // See <https://doc.rust-lang.org/reference/items/functions.html> and related grammar.
        //
        // Should be removable in the near future: <https://github.com/rust-lang/rfcs/pull/3399>.
        #[cfg(feature = "output_progress")] FnExec: for<'f_exec> FnOnce(
            &'f_exec mut SingleProfileSingleFlowView<'_, E, PKeys, SetUp>,
            &'f_exec Sender<ProgressUpdateAndId>,
            &'f_exec UnboundedSender<ItemOutcomeT>,
        ) -> LocalBoxFuture<'f_exec, ()>,
        #[cfg(not(feature = "output_progress"))] FnExec: for<'f_exec> FnOnce(
            &'f_exec mut SingleProfileSingleFlowView<'_, E, PKeys, SetUp>,
            &'f_exec UnboundedSender<ItemOutcomeT>,
        ) -> LocalBoxFuture<'f_exec, ()>,
        OutcomeT,
        FnOutcomeCollate,
    >(
        cmd_independence: &mut CmdIndependence<'_, '_, '_, E, O, PKeys>,
        outcome: OutcomeT,
        fn_exec: FnExec,
        fn_outcome: FnOutcomeCollate,
    ) -> Result<CmdOutcome<OutcomeT, E>, E>
    where
        ItemOutcomeT: Send + 'static,
        OutcomeT: 'static,
        FnOutcomeCollate: Fn(&mut CmdOutcome<OutcomeT, E>, ItemOutcomeT) -> Result<(), E>,
    {
        let (outcomes_tx, mut outcomes_rx) = mpsc::unbounded_channel::<ItemOutcomeT>();
        let mut cmd_outcome = {
            let errors = IndexMap::<ItemId, E>::new();
            CmdOutcome {
                value: outcome,
                errors,
            }
        };
        let outcomes_rx_task = async {
            while let Some(item_outcome) = outcomes_rx.recv().await {
                fn_outcome(&mut cmd_outcome, item_outcome)?;
            }

            Result::<(), E>::Ok(())
        };

        cfg_if::cfg_if! {
            if #[cfg(feature = "output_progress")] {
                match cmd_independence {
                    CmdIndependence::Standalone { cmd_ctx } => {
                        let SingleProfileSingleFlowViewAndOutput {
                            output,
                            cmd_progress_tracker,
                            mut cmd_view,
                            ..
                        } = cmd_ctx.view_and_output();

                        output.progress_begin(cmd_progress_tracker).await;

                        let (progress_tx, progress_rx) =
                            mpsc::channel::<ProgressUpdateAndId>(crate::PROGRESS_COUNT_MAX);
                        let progress_render_task ={
                            let progress_trackers = &mut cmd_progress_tracker.progress_trackers;
                            crate::progress::Progress::progress_render(
                                output,
                                progress_trackers,
                                progress_rx)
                        };

                        let execution_task = async move {
                            let progress_tx = &progress_tx;
                            let outcomes_tx = &outcomes_tx;

                            fn_exec(&mut cmd_view, progress_tx, outcomes_tx).await;

                            // `progress_tx` is dropped here, so `progress_rx` will safely end.
                        };

                        let ((), (), outcome_result) = futures::join!(execution_task, progress_render_task, outcomes_rx_task);
                        output.progress_end(cmd_progress_tracker).await;

                        outcome_result?;
                    }
                    CmdIndependence::SubCmd { cmd_view } => {
                        // Dud progress_tx that discards everything.
                        let (progress_tx, mut progress_rx) =
                            mpsc::channel::<ProgressUpdateAndId>(crate::PROGRESS_COUNT_MAX);

                        let progress_render_task = async move {
                            while progress_rx.recv().await.is_some() {}
                        };

                        let execution_task = async move {
                            let progress_tx = &progress_tx;
                            let outcomes_tx = &outcomes_tx;

                            fn_exec(cmd_view, progress_tx, outcomes_tx).await;

                            // `progress_tx` is dropped here, so `progress_rx` will safely end.
                        };

                        let ((), (), outcome_result) = futures::join!(execution_task, progress_render_task, outcomes_rx_task);

                        outcome_result?;
                    }
                    CmdIndependence::SubCmdWithProgress {
                        cmd_view,
                        progress_tx,
                    } => {
                        let execution_task = async move {
                            let progress_tx = &progress_tx;
                            let outcomes_tx = &outcomes_tx;

                            fn_exec(cmd_view, progress_tx, outcomes_tx).await;

                            // `progress_tx` is dropped here, so `progress_rx` will safely end.
                        };

                        let ((), outcome_result) = futures::join!(execution_task, outcomes_rx_task);
                        outcome_result?;
                    }
                }

            } else {
                match cmd_independence {
                    CmdIndependence::Standalone { cmd_ctx } => {
                        let mut cmd_view = cmd_ctx.view();

                        let execution_task = async move {
                            let outcomes_tx = &outcomes_tx;
                            fn_exec(&mut cmd_view, outcomes_tx).await;
                        };

                        let ((), outcome_result) = futures::join!(execution_task, outcomes_rx_task);
                        outcome_result?;
                    }
                    CmdIndependence::SubCmd {
                        cmd_view,
                    } => {
                        let execution_task = async move {
                            let outcomes_tx = &outcomes_tx;
                            fn_exec(cmd_view, outcomes_tx).await;
                        };

                        let ((), outcome_result) = futures::join!(execution_task, outcomes_rx_task);
                        outcome_result?;
                    }
                }
            }
        }

        Ok(cmd_outcome)
    }
}

impl<E, O, PKeys> Default for CmdBase<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
