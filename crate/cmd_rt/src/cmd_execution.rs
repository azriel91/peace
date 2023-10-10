use std::{collections::VecDeque, fmt::Debug, pin::Pin};

use futures::{future, stream, Future, StreamExt, TryStreamExt};
use interruptible::{InterruptSignal, InterruptibleFutureExt};
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{
        SingleProfileSingleFlow, SingleProfileSingleFlowView, SingleProfileSingleFlowViewAndOutput,
    },
};
use peace_resources::{resources::ts::SetUp, Resources};
use peace_rt_model::{outcomes::CmdOutcome, output::OutputWrite, params::ParamsKeys};
use tokio::sync::mpsc;

use crate::{CmdBlockError, CmdBlockRt, CmdBlockRtBox};

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::progress::{
            ProgressUpdateAndId,
        };
        use peace_rt_model::CmdProgressTracker;
        use tokio::sync::mpsc::Sender;

        use crate::Progress;
    }
}

pub use self::{
    cmd_execution_builder::CmdExecutionBuilder,
    cmd_execution_error_builder::CmdExecutionErrorBuilder,
    interruptibility::{Interruptible, InterruptibleT, NonInterruptible},
};

mod cmd_execution_builder;
mod cmd_execution_error_builder;
mod interruptibility;

/// List of [`CmdBlock`]s to run for a `*Cmd`.
///
/// A `CmdExecution` is interruptible if [`CmdExecutionBuilder::interruptible`]
/// is called during construction.
///
/// # Design
///
/// Interruptibility is implemented as type state. It could be implemented as a
/// feature flag as well; I don't know if developers want certain command
/// executions to be interruptible, and others not.
///
/// [`CmdBlock`]: crate::CmdBlock
#[derive(Debug)]
pub struct CmdExecution<ExecutionOutcome, E, PKeys>
where
    E: 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Blocks of commands to run.
    cmd_blocks: VecDeque<CmdBlockRtBox<E, PKeys, ExecutionOutcome>>,
    /// Logic to extract the `ExecutionOutcome` from `Resources`.
    execution_outcome_fetch: fn(&mut Resources<SetUp>) -> ExecutionOutcome,
    /// Whether or not to render progress.
    #[cfg(feature = "output_progress")]
    progress_render_enabled: bool,
}

impl<ExecutionOutcome, E, PKeys> CmdExecution<ExecutionOutcome, E, PKeys>
where
    ExecutionOutcome: Debug + Send + Sync + Unpin + 'static,
    E: std::error::Error + From<peace_rt_model::Error> + Send + Sync + Unpin + 'static,
    PKeys: ParamsKeys + 'static,
{
    pub fn builder() -> CmdExecutionBuilder<ExecutionOutcome, E, PKeys> {
        CmdExecutionBuilder::new()
    }

    /// Returns the result of executing the command.
    pub async fn exec<O>(
        &mut self,
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<CmdOutcome<ExecutionOutcome, E>, E>
    where
        O: OutputWrite<E>,
    {
        let Self {
            cmd_blocks,
            execution_outcome_fetch,
            #[cfg(feature = "output_progress")]
            progress_render_enabled,
        } = self;
        #[cfg(feature = "output_progress")]
        let progress_render_enabled = *progress_render_enabled;

        cfg_if::cfg_if! {
            if #[cfg(feature = "output_progress")] {
                let SingleProfileSingleFlowViewAndOutput {
                    output,
                    interrupt_rx,
                    cmd_progress_tracker,
                    mut cmd_view,
                    ..
                } = cmd_ctx.view_and_output();

                let (progress_tx, progress_rx) =
                    mpsc::channel::<ProgressUpdateAndId>(crate::PROGRESS_COUNT_MAX);
            } else {
                let SingleProfileSingleFlowViewAndOutput {
                    interrupt_rx,
                    mut cmd_view,
                    ..
                } = cmd_ctx.view_and_output();
            }
        }

        if let Some(interrupt_rx) = interrupt_rx {
            let cmd_outcome_task = cmd_outcome_task_interruptible(
                cmd_blocks,
                execution_outcome_fetch,
                &mut cmd_view,
                interrupt_rx,
                #[cfg(feature = "output_progress")]
                progress_tx,
            );

            exec_internal(
                cmd_outcome_task,
                #[cfg(feature = "output_progress")]
                progress_render_enabled,
                #[cfg(feature = "output_progress")]
                output,
                #[cfg(feature = "output_progress")]
                cmd_progress_tracker,
                #[cfg(feature = "output_progress")]
                progress_rx,
            )
            .await
        } else {
            let cmd_outcome_task = cmd_outcome_task(
                cmd_blocks,
                execution_outcome_fetch,
                &mut cmd_view,
                #[cfg(feature = "output_progress")]
                progress_tx,
            );

            exec_internal(
                cmd_outcome_task,
                #[cfg(feature = "output_progress")]
                progress_render_enabled,
                #[cfg(feature = "output_progress")]
                output,
                #[cfg(feature = "output_progress")]
                cmd_progress_tracker,
                #[cfg(feature = "output_progress")]
                progress_rx,
            )
            .await
        }
    }

    // pub fn exec_bg -> CmdExecId
}

/// Executes and returns the `CmdOutcome`.
///
/// This also runs the progress task if the `"output_progress"` feature is
/// enabled.
async fn exec_internal<ExecutionOutcome, E, #[cfg(feature = "output_progress")] O: OutputWrite<E>>(
    cmd_outcome_task: impl Future<Output = Result<CmdOutcome<ExecutionOutcome, E>, E>>,
    #[cfg(feature = "output_progress")] progress_render_enabled: bool,
    #[cfg(feature = "output_progress")] output: &mut O,
    #[cfg(feature = "output_progress")] cmd_progress_tracker: &mut CmdProgressTracker,
    #[cfg(feature = "output_progress")] mut progress_rx: mpsc::Receiver<ProgressUpdateAndId>,
) -> Result<CmdOutcome<ExecutionOutcome, E>, E>
where
    ExecutionOutcome: Debug + Send + Sync + Unpin + 'static,
    E: std::error::Error + From<peace_rt_model::Error> + Send + Sync + Unpin + 'static,
{
    #[cfg(not(feature = "output_progress"))]
    {
        cmd_outcome_task.await
    }

    #[cfg(feature = "output_progress")]
    if progress_render_enabled {
        output.progress_begin(cmd_progress_tracker).await;
        let progress_trackers = &mut cmd_progress_tracker.progress_trackers;
        let progress_render_task =
            Progress::progress_render(output, progress_trackers, progress_rx);

        let (cmd_outcome, ()) = futures::join!(cmd_outcome_task, progress_render_task);

        output.progress_end(cmd_progress_tracker).await;

        cmd_outcome
    } else {
        // When `progress_render_enabled` is false, still consumes progress updates
        // and drop them.
        let progress_render_task = async move { while progress_rx.recv().await.is_some() {} };

        let (cmd_outcome, ()) = futures::join!(cmd_outcome_task, progress_render_task);

        cmd_outcome
    }
}

async fn cmd_outcome_task<'view: 'scope, 'scope, ExecutionOutcome, E, PKeys>(
    cmd_blocks: &'view VecDeque<CmdBlockRtBox<E, PKeys, ExecutionOutcome>>,
    execution_outcome_fetch: &'view mut fn(&mut Resources<SetUp>) -> ExecutionOutcome,
    cmd_view: &'view mut SingleProfileSingleFlowView<'scope, E, PKeys, SetUp>,
    #[cfg(feature = "output_progress")] progress_tx: Sender<ProgressUpdateAndId>,
) -> Result<CmdOutcome<ExecutionOutcome, E>, E>
where
    E: std::error::Error + From<peace_rt_model::Error> + Send + Sync + Unpin + 'static,
    PKeys: ParamsKeys + 'static,
    ExecutionOutcome: Debug + Send + Sync + Unpin + 'static,
{
    let cmd_view_and_progress_result = stream::unfold(cmd_blocks.iter(), |mut cmd_blocks| {
        let cmd_block_next = cmd_blocks.next();
        future::ready(cmd_block_next.map(|cmd_block_next| (cmd_block_next, cmd_blocks)))
    })
    .enumerate()
    .map(Result::<_, CmdViewAndErr<ExecutionOutcome, E, PKeys>>::Ok)
    .try_fold(
        CmdViewAndProgress {
            cmd_view,
            #[cfg(feature = "output_progress")]
            progress_tx,
        },
        // `progress_tx` is moved into this closure, and dropped at the very end, so
        // that `progress_render_task` will actually end.
        |cmd_view_and_progress, (cmd_block_index, cmd_block_rt)| async move {
            let CmdViewAndProgress {
                cmd_view,
                #[cfg(feature = "output_progress")]
                progress_tx,
            } = cmd_view_and_progress;

            let block_cmd_outcome_result = cmd_block_rt
                .exec(
                    cmd_view,
                    #[cfg(feature = "output_progress")]
                    progress_tx.clone(),
                )
                .await;

            // `CmdBlock` block logic errors are propagated.
            let cmd_view_and_progress = CmdViewAndProgress {
                cmd_view,
                #[cfg(feature = "output_progress")]
                progress_tx,
            };

            match block_cmd_outcome_result {
                Ok(()) => Ok(cmd_view_and_progress),
                Err(cmd_block_error) => Err(CmdViewAndErr {
                    cmd_view_and_progress,
                    cmd_block_index,
                    cmd_block_error,
                }),
            }
        },
    )
    .await;

    outcome_extract::<ExecutionOutcome, E, PKeys>(
        cmd_view_and_progress_result,
        cmd_blocks,
        execution_outcome_fetch,
    )
}

async fn cmd_outcome_task_interruptible<'view: 'scope, 'scope, ExecutionOutcome, E, PKeys>(
    cmd_blocks: &'view VecDeque<CmdBlockRtBox<E, PKeys, ExecutionOutcome>>,
    execution_outcome_fetch: &'view mut fn(&mut Resources<SetUp>) -> ExecutionOutcome,
    cmd_view: &'view mut SingleProfileSingleFlowView<'scope, E, PKeys, SetUp>,
    interrupt_rx: &'scope mut mpsc::Receiver<InterruptSignal>,
    #[cfg(feature = "output_progress")] progress_tx: Sender<ProgressUpdateAndId>,
) -> Result<CmdOutcome<ExecutionOutcome, E>, E>
where
    E: std::error::Error + From<peace_rt_model::Error> + Send + Sync + Unpin + 'static,
    PKeys: ParamsKeys + 'static,
    ExecutionOutcome: Debug + Send + Sync + Unpin + 'static,
    CmdBlockError<ExecutionOutcome, E>: From<((), InterruptSignal)>,
{
    let cmd_view_and_progress_interruptible_result: Result<
        CmdViewAndProgressInterruptible<'_, '_, _, _>,
        CmdViewAndErrInterruptible<'_, '_, _, _, _>,
    > = stream::unfold(cmd_blocks.iter(), |mut cmd_blocks| {
        let cmd_block_next = cmd_blocks.next();
        future::ready(cmd_block_next.map(|cmd_block_next| (cmd_block_next, cmd_blocks)))
    })
    .enumerate()
    .map(Result::<_, CmdViewAndErrInterruptible<ExecutionOutcome, E, PKeys>>::Ok)
    .try_fold(
        CmdViewAndProgressInterruptible {
            cmd_view,
            interrupt_rx,
            #[cfg(feature = "output_progress")]
            progress_tx,
        },
        // `progress_tx` is moved into this closure, and dropped at the very end, so
        // that `progress_render_task` will actually end.
        |cmd_view_and_progress_interruptible, (cmd_block_index, cmd_block_rt)| async move {
            let CmdViewAndProgressInterruptible {
                cmd_view,
                interrupt_rx,
                #[cfg(feature = "output_progress")]
                progress_tx,
            } = cmd_view_and_progress_interruptible;

            let block_cmd_outcome_result = cmd_block_rt
                .exec(
                    cmd_view,
                    #[cfg(feature = "output_progress")]
                    progress_tx.clone(),
                )
                .interruptible_result(interrupt_rx)
                .await;

            // `CmdBlock` block logic errors are propagated.
            let cmd_view_and_progress_interruptible = CmdViewAndProgressInterruptible {
                cmd_view,
                interrupt_rx,
                #[cfg(feature = "output_progress")]
                progress_tx,
            };

            match block_cmd_outcome_result {
                Ok(()) => Ok(cmd_view_and_progress_interruptible),
                Err(cmd_block_error) => Err(CmdViewAndErrInterruptible {
                    cmd_view_and_progress_interruptible,
                    cmd_block_index,
                    cmd_block_error,
                }),
            }
        },
    )
    .await;

    let cmd_view_and_progress_result = cmd_view_and_progress_interruptible_result
        .map(CmdViewAndProgress::from)
        .map_err(CmdViewAndErr::from);

    outcome_extract::<ExecutionOutcome, E, PKeys>(
        cmd_view_and_progress_result,
        cmd_blocks,
        execution_outcome_fetch,
    )
}

/// Extracts the `ExecutionOutcome` from the intermediate outcome collating
/// types.
///
/// # Parameters
///
/// * `cmd_view_and_progress_result`: The command context, progress, and maybe
///   error.
/// * `cmd_blocks`: `CmdBlock`s in this execution, used to build a useful error
///   message if needed.
/// * `execution_outcome_fetch`: Logic to extract the `ExecutionOutcome` type.
fn outcome_extract<'view_ref: 'view, 'view, ExecutionOutcome, E, PKeys>(
    cmd_view_and_progress_result: Result<
        CmdViewAndProgress<'view_ref, 'view, E, PKeys>,
        CmdViewAndErr<'view_ref, 'view, ExecutionOutcome, E, PKeys>,
    >,
    cmd_blocks: &'view_ref VecDeque<
        Pin<Box<dyn CmdBlockRt<Error = E, PKeys = PKeys, ExecutionOutcome = ExecutionOutcome>>>,
    >,
    execution_outcome_fetch: &mut fn(&mut Resources<SetUp>) -> ExecutionOutcome,
) -> Result<CmdOutcome<ExecutionOutcome, E>, E>
where
    E: std::error::Error + From<peace_rt_model::Error> + Send + Sync + Unpin + 'static,
    PKeys: ParamsKeys + 'static,
    ExecutionOutcome: Debug + Send + Sync + Unpin + 'static,
{
    let (cmd_view_and_progress, cmd_block_index_and_error) = match cmd_view_and_progress_result {
        Ok(cmd_view_and_progress) => (cmd_view_and_progress, None),
        Err(CmdViewAndErr {
            cmd_view_and_progress,
            cmd_block_index,
            cmd_block_error,
        }) => (
            cmd_view_and_progress,
            Some((cmd_block_index, cmd_block_error)),
        ),
    };

    let CmdViewAndProgress {
        cmd_view,
        #[cfg(feature = "output_progress")]
        progress_tx,
    } = cmd_view_and_progress;

    #[cfg(feature = "output_progress")]
    drop(progress_tx);

    if let Some((cmd_block_index, cmd_block_error)) = cmd_block_index_and_error {
        match cmd_block_error {
            CmdBlockError::InputFetch(resource_fetch_error) => {
                Err(CmdExecutionErrorBuilder::build::<_, _, _, _>(
                    cmd_blocks.iter(),
                    cmd_block_index,
                    resource_fetch_error,
                ))
                .map_err(peace_rt_model::Error::from)
                .map_err(E::from)
            }
            CmdBlockError::Block(error) => Err(error),
            CmdBlockError::Outcome(cmd_outcome) => Ok(cmd_outcome),
            CmdBlockError::Interrupt => {
                let execution_outcome = execution_outcome_fetch(cmd_view.resources);
                let cmd_outcome = CmdOutcome::new(execution_outcome);
                Ok(cmd_outcome)
            }
        }
    } else {
        let execution_outcome = execution_outcome_fetch(cmd_view.resources);
        let cmd_outcome = CmdOutcome::new(execution_outcome);
        Ok(cmd_outcome)
    }
}

struct CmdViewAndProgress<'view_ref: 'view, 'view, E, PKeys>
where
    E: 'static,
    PKeys: ParamsKeys + 'static,
{
    cmd_view: &'view_ref mut SingleProfileSingleFlowView<'view, E, PKeys, SetUp>,
    #[cfg(feature = "output_progress")]
    progress_tx: Sender<ProgressUpdateAndId>,
}

struct CmdViewAndProgressInterruptible<'view_ref: 'view, 'view, E, PKeys>
where
    E: 'static,
    PKeys: ParamsKeys + 'static,
{
    cmd_view: &'view_ref mut SingleProfileSingleFlowView<'view, E, PKeys, SetUp>,
    interrupt_rx: &'view_ref mut mpsc::Receiver<InterruptSignal>,
    #[cfg(feature = "output_progress")]
    progress_tx: Sender<ProgressUpdateAndId>,
}

impl<'view_ref: 'view, 'view, E, PKeys>
    From<CmdViewAndProgressInterruptible<'view_ref, 'view, E, PKeys>>
    for CmdViewAndProgress<'view_ref, 'view, E, PKeys>
where
    E: Debug + 'static,
    PKeys: ParamsKeys + 'static,
{
    fn from(
        cmd_view_and_progress_interruptible: CmdViewAndProgressInterruptible<
            'view_ref,
            'view,
            E,
            PKeys,
        >,
    ) -> Self {
        let CmdViewAndProgressInterruptible {
            cmd_view,
            interrupt_rx: _,
            #[cfg(feature = "output_progress")]
            progress_tx,
        } = cmd_view_and_progress_interruptible;

        CmdViewAndProgress {
            cmd_view,
            #[cfg(feature = "output_progress")]
            progress_tx,
        }
    }
}

struct CmdViewAndErr<'view_ref: 'view, 'view, ExecutionOutcome, E, PKeys>
where
    ExecutionOutcome: Debug,
    E: Debug + 'static,
    PKeys: ParamsKeys + 'static,
{
    cmd_view_and_progress: CmdViewAndProgress<'view_ref, 'view, E, PKeys>,
    cmd_block_index: usize,
    cmd_block_error: CmdBlockError<ExecutionOutcome, E>,
}

struct CmdViewAndErrInterruptible<'view_ref: 'view, 'view, ExecutionOutcome, E, PKeys>
where
    ExecutionOutcome: Debug,
    E: Debug + 'static,
    PKeys: ParamsKeys + 'static,
{
    cmd_view_and_progress_interruptible:
        CmdViewAndProgressInterruptible<'view_ref, 'view, E, PKeys>,
    cmd_block_index: usize,
    cmd_block_error: CmdBlockError<ExecutionOutcome, E>,
}

impl<'view_ref: 'view, 'view, ExecutionOutcome, E, PKeys>
    From<CmdViewAndErrInterruptible<'view_ref, 'view, ExecutionOutcome, E, PKeys>>
    for CmdViewAndErr<'view_ref, 'view, ExecutionOutcome, E, PKeys>
where
    ExecutionOutcome: Debug,
    E: Debug + 'static,
    PKeys: ParamsKeys + 'static,
{
    fn from(
        cmd_view_and_err_interruptible: CmdViewAndErrInterruptible<
            'view_ref,
            'view,
            ExecutionOutcome,
            E,
            PKeys,
        >,
    ) -> Self {
        let CmdViewAndErrInterruptible {
            cmd_view_and_progress_interruptible,
            cmd_block_index,
            cmd_block_error,
        } = cmd_view_and_err_interruptible;

        let cmd_view_and_progress = CmdViewAndProgress::from(cmd_view_and_progress_interruptible);

        CmdViewAndErr {
            cmd_view_and_progress,
            cmd_block_index,
            cmd_block_error,
        }
    }
}
