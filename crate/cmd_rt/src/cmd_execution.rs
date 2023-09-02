use std::{collections::VecDeque, fmt::Debug};

use futures::{future, stream, StreamExt, TryStreamExt};
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::resources::ts::SetUp;
use peace_rt_model::{outcomes::CmdOutcome, output::OutputWrite, params::ParamsKeys};

use crate::{CmdBlockError, CmdBlockRtBox};

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cmd::scopes::SingleProfileSingleFlowViewAndOutput;
        use peace_cfg::progress::ProgressUpdateAndId;
        use tokio::sync::mpsc::{self, Sender};

        use crate::Progress;
    }
}

pub use self::cmd_execution_builder::CmdExecutionBuilder;

mod cmd_execution_builder;

/// List of [`CmdBlock`]s to run for a `*Cmd`.
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
}

impl<ExecutionOutcome, E, PKeys> CmdExecution<ExecutionOutcome, E, PKeys>
where
    ExecutionOutcome: Debug + Send + Sync + 'static,
    E: std::error::Error + From<peace_rt_model::Error> + Send + Sync + Unpin + 'static,
    PKeys: Debug + ParamsKeys + Unpin + 'static,
{
    pub fn builder() -> CmdExecutionBuilder<ExecutionOutcome, E, PKeys> {
        CmdExecutionBuilder::new()
    }
}

impl<ExecutionOutcome, E, PKeys> CmdExecution<ExecutionOutcome, E, PKeys>
where
    E: std::error::Error + From<peace_rt_model::Error> + Send + Sync + Unpin + 'static,
    PKeys: Debug + ParamsKeys + Unpin + 'static,
    ExecutionOutcome: Debug + Send + Sync + Unpin + 'static,
{
    /// Returns the result of executing the command.
    pub async fn exec<O>(
        &mut self,
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<CmdOutcome<ExecutionOutcome, E>, E>
    where
        O: OutputWrite<E>,
    {
        let Self { cmd_blocks } = self;

        cfg_if::cfg_if! {
            if #[cfg(feature = "output_progress")] {
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
                    Progress::progress_render(
                        output,
                        progress_trackers,
                        progress_rx)
                };
            } else {
                let mut cmd_view = cmd_ctx.view();
            }
        }

        let cmd_outcome_task = async {
            let cmd_view_and_progress_result = stream::unfold(cmd_blocks, |cmd_blocks| {
                let cmd_block_next = cmd_blocks.pop_front();
                future::ready(cmd_block_next.map(|cmd_block_next| (cmd_block_next, cmd_blocks)))
            })
            .map(Result::<_, CmdViewAndErr<ExecutionOutcome, E, PKeys>>::Ok)
            .try_fold(
                CmdViewAndProgress {
                    cmd_view: &mut cmd_view,
                    #[cfg(feature = "output_progress")]
                    progress_tx,
                },
                // `progress_tx` is moved into this closure, and dropped at the very end, so
                // that `progress_render_task` will actually end.
                |cmd_view_and_progress, cmd_block_rt| async move {
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
                            cmd_block_error,
                        }),
                    }
                },
            )
            .await;

            let (cmd_view_and_progress, cmd_block_error) = match cmd_view_and_progress_result {
                Ok(cmd_view_and_progress) => (cmd_view_and_progress, None),
                Err(CmdViewAndErr {
                    cmd_view_and_progress,
                    cmd_block_error,
                }) => (cmd_view_and_progress, Some(cmd_block_error)),
            };

            let CmdViewAndProgress {
                cmd_view,
                #[cfg(feature = "output_progress")]
                progress_tx,
            } = cmd_view_and_progress;

            #[cfg(feature = "output_progress")]
            drop(progress_tx);

            if let Some(cmd_block_error) = cmd_block_error {
                match cmd_block_error {
                    CmdBlockError::Block(error) => Err(error),
                    CmdBlockError::Outcome(cmd_outcome) => Ok(cmd_outcome),
                }
            } else {
                let execution_outcome = cmd_view
                    .resources
                    .remove::<ExecutionOutcome>()
                    .unwrap_or_else(|| {
                        let execution_outcome_type_name = tynm::type_name::<ExecutionOutcome>();
                        panic!(
                            "Expected `{execution_outcome_type_name}` to exist in `Resources`.\n\
                            Make sure the final `CmdBlock` has that type as its `Outcome`."
                        );
                    });
                let cmd_outcome = CmdOutcome::new(execution_outcome);
                Ok(cmd_outcome)
            }
        };

        #[cfg(not(feature = "output_progress"))]
        let cmd_outcome = cmd_outcome_task.await;
        #[cfg(feature = "output_progress")]
        let (cmd_outcome, ()) = futures::join!(cmd_outcome_task, progress_render_task);

        #[cfg(feature = "output_progress")]
        output.progress_end(cmd_progress_tracker).await;

        cmd_outcome
    }

    // pub fn exec_bg -> CmdExecId
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

struct CmdViewAndErr<'view_ref: 'view, 'view, ExecutionOutcome, E, PKeys>
where
    ExecutionOutcome: Debug,
    E: Debug + 'static,
    PKeys: Debug + ParamsKeys + 'static,
{
    cmd_view_and_progress: CmdViewAndProgress<'view_ref, 'view, E, PKeys>,
    cmd_block_error: CmdBlockError<ExecutionOutcome, E>,
}
