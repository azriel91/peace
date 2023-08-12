use std::{collections::VecDeque, fmt::Debug};

use futures::{future, stream, StreamExt, TryStreamExt};
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{resources::ts::SetUp, Resource};
use peace_rt_model::{outcomes::CmdOutcome, output::OutputWrite, params::ParamsKeys};

use crate::CmdBlockRtBox;

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
pub struct CmdExecution<E, PKeys, Outcome>
where
    E: 'static,
    PKeys: ParamsKeys + 'static,
{
    /// Blocks of commands to run.
    cmd_blocks: VecDeque<CmdBlockRtBox<E, PKeys, Outcome>>,
}

impl<E, PKeys, Outcome> CmdExecution<E, PKeys, Outcome>
where
    E: std::error::Error + From<peace_rt_model::Error> + Send + Sync + Unpin + 'static,
    PKeys: Debug + ParamsKeys + Unpin + 'static,
    Outcome: Debug + Send + Sync + Unpin + 'static,
{
    pub fn builder() -> CmdExecutionBuilder<E, PKeys, Outcome> {
        CmdExecutionBuilder::new()
    }

    /// Returns the result of executing the command.
    pub async fn exec<O>(
        &mut self,
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<Box<CmdOutcome<Outcome, E>>, E>
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
            let CmdViewAndBlockOutcome {
                cmd_view: _cmd_view,
                cmd_outcome,
                #[cfg(feature = "output_progress")]
                progress_tx,
            } = stream::unfold(cmd_blocks, |cmd_blocks| {
                let cmd_block_next = cmd_blocks.pop_front();
                future::ready(cmd_block_next.map(|cmd_block_next| (cmd_block_next, cmd_blocks)))
            })
            .map(Result::<_, E>::Ok)
            .try_fold(
                CmdViewAndBlockOutcome {
                    cmd_view: &mut cmd_view,
                    cmd_outcome: Box::new(()) as Box<dyn Resource>,
                    #[cfg(feature = "output_progress")]
                    progress_tx,
                },
                // `progress_tx` is moved into this closure, and dropped at the very end, so
                // that `progress_render_task` will actually end.
                |cmd_view_and_block_outcome, cmd_block_rt| async move {
                    let CmdViewAndBlockOutcome {
                        cmd_view,
                        cmd_outcome: cmd_outcome_previous,
                        #[cfg(feature = "output_progress")]
                        progress_tx,
                    } = cmd_view_and_block_outcome;

                    let block_cmd_outcome_task = cmd_block_rt.exec(
                        cmd_view,
                        #[cfg(feature = "output_progress")]
                        progress_tx.clone(),
                        cmd_outcome_previous,
                    );

                    let block_cmd_outcome = block_cmd_outcome_task.await;

                    block_cmd_outcome.map(|block_cmd_outcome| CmdViewAndBlockOutcome {
                        cmd_view,
                        cmd_outcome: Box::new(block_cmd_outcome) as Box<dyn Resource>,
                        #[cfg(feature = "output_progress")]
                        progress_tx,
                    })
                },
            )
            .await?;

            #[cfg(feature = "output_progress")]
            drop(progress_tx);

            Result::<_, E>::Ok(cmd_outcome)
        };

        #[cfg(not(feature = "output_progress"))]
        let cmd_outcome = cmd_outcome_task.await;
        #[cfg(feature = "output_progress")]
        let (cmd_outcome, ()) = futures::join!(cmd_outcome_task, progress_render_task);

        #[cfg(feature = "output_progress")]
        output.progress_end(cmd_progress_tracker).await;

        let cmd_outcome = cmd_outcome?.downcast().unwrap_or_else(|cmd_outcome| {
            let outcome_type_name = tynm::type_name::<Outcome>();
            let actual_type_name = Resource::type_name(&*cmd_outcome);
            panic!(
                "Expected to downcast `cmd_outcome` to `{outcome_type_name}`.\n\
                The actual type name is `{actual_type_name:?}`\n\
                This is a bug in the Peace framework."
            );
        });

        Ok(cmd_outcome)
    }

    // pub fn exec_bg -> CmdExecId
}

struct CmdViewAndBlockOutcome<'view_ref: 'view, 'view, E, PKeys>
where
    E: 'static,
    PKeys: ParamsKeys + 'static,
{
    cmd_view: &'view_ref mut SingleProfileSingleFlowView<'view, E, PKeys, SetUp>,
    cmd_outcome: Box<dyn Resource>,
    #[cfg(feature = "output_progress")]
    progress_tx: Sender<ProgressUpdateAndId>,
}
