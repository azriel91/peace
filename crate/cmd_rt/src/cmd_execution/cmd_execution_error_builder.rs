use std::fmt::Debug;

use peace_cmd_model::{CmdBlockDesc, CmdExecutionError};
use peace_resources::ResourceFetchError;
use peace_rt_model::params::ParamsKeys;

use crate::CmdBlockRtBox;

cfg_if::cfg_if! {
    if #[cfg(feature = "error_reporting")] {
        use std::fmt::{self, Write};

        use tynm::TypeParamsFmtOpts;
        use miette::SourceSpan;

        use crate::CmdExecution;
    }
}

/// Computes the values to construct a `CmdExecutionError`.
#[derive(Debug)]
pub struct CmdExecutionErrorBuilder;

impl CmdExecutionErrorBuilder {
    /// Returns a `CmdExecutionError` by collating `CmdBlock` information.
    ///
    /// Approximation of the source for `EnsureCmd`:
    ///
    /// ```yaml
    /// CmdExecution:
    ///   ExecutionOutcome: (States<Previous>, States<Ensured>, States<Goal>)
    /// CmdBlocks:
    ///   - StatesCurrentReadCmdBlock:
    ///       Input: States<Current>
    ///       Outcome: States<Goal>
    ///   - StatesGoalReadCmdBlock:
    ///       Input: States<Current>
    ///       Outcome: States<Goal>
    ///   - StatesDiscoverCmdBlock:
    ///       Input: ()
    ///       Outcome: (States<Current>, States<Goal>)
    ///   - ApplyStateSyncCheckCmdBlock:
    ///       Input: (States<CurrentStored>, States<Current>, States<GoalStored>, States<Goal>)
    ///       Outcome: (States<CurrentStored>, States<Current>, States<GoalStored>, States<Goal>)
    ///   - ApplyExecCmdBlock:
    ///       Input: (States<Current>, States<Goal>)
    ///       Outcome: (States<Previous>, States<Ensured>, States<Goal>)
    /// ```
    pub fn build<'f, ExecutionOutcome, E, PKeys, CmdBlockIterator>(
        cmd_blocks: CmdBlockIterator,
        cmd_block_index: usize,
        resource_fetch_error: ResourceFetchError,
    ) -> CmdExecutionError
    where
        E: std::error::Error + From<peace_rt_model::Error> + Send + Sync + Unpin + 'static,
        PKeys: ParamsKeys + 'static,
        ExecutionOutcome: Debug + Send + Sync + Unpin + 'static,
        CmdBlockIterator: Iterator<Item = &'f CmdBlockRtBox<E, PKeys, ExecutionOutcome>>,
    {
        let ResourceFetchError {
            resource_name_short: input_name_short,
            resource_name_full: input_name_full,
        } = resource_fetch_error;

        let cmd_block_descs = cmd_blocks
            .map(|cmd_block_rt| cmd_block_rt.cmd_block_desc())
            .collect::<Vec<CmdBlockDesc>>();

        #[cfg(feature = "error_reporting")]
        let (cmd_execution_src, input_span) =
            cmd_execution_src::<ExecutionOutcome, E, PKeys>(&cmd_block_descs, &input_name_short)
                .expect("Failed to write to `cmd_execution_src` buffer.");

        CmdExecutionError::InputFetch {
            cmd_block_descs,
            cmd_block_index,
            input_name_short,
            input_name_full,
            #[cfg(feature = "error_reporting")]
            cmd_execution_src,
            #[cfg(feature = "error_reporting")]
            input_span,
        }
    }
}

#[cfg(feature = "error_reporting")]
fn cmd_execution_src<ExecutionOutcome, E, PKeys>(
    cmd_block_descs: &[CmdBlockDesc],
    input_name_short: &str,
) -> Result<(String, Option<SourceSpan>), fmt::Error>
where
    E: std::error::Error + From<peace_rt_model::Error> + Send + Sync + Unpin + 'static,
    PKeys: ParamsKeys + 'static,
    ExecutionOutcome: Debug + Send + Sync + Unpin + 'static,
{
    let mut cmd_execution_src = String::with_capacity(2048);

    let cmd_execution_name =
        tynm::type_name_opts::<CmdExecution<ExecutionOutcome, E, PKeys>>(TypeParamsFmtOpts::Std);
    let execution_outcome_types_name =
        tynm::type_name_opts::<ExecutionOutcome>(TypeParamsFmtOpts::All);

    writeln!(&mut cmd_execution_src, "{cmd_execution_name}:")?;
    writeln!(
        &mut cmd_execution_src,
        "  ExecutionOutcome: {execution_outcome_types_name}"
    )?;
    writeln!(&mut cmd_execution_src, "CmdBlocks:")?;

    let input_span =
        cmd_block_descs
            .iter()
            .try_fold(None, |mut input_span_opt, cmd_block_desc| {
                let cmd_block_name = cmd_block_desc.cmd_block_name();
                writeln!(&mut cmd_execution_src, "  - {cmd_block_name}:")?;

                write!(&mut cmd_execution_src, "    Input: ")?;
                match cmd_block_desc.cmd_block_input_names().split_first() {
                    None => writeln!(&mut cmd_execution_src, "()")?,
                    Some((input_first, input_remainder)) => {
                        if input_remainder.is_empty() {
                            if input_first == input_name_short {
                                input_span_opt = Some(SourceSpan::from((
                                    cmd_execution_src.len(),
                                    input_first.len(),
                                )));
                            }
                            writeln!(&mut cmd_execution_src, "{input_first}")?;
                        } else {
                            write!(&mut cmd_execution_src, "(")?;
                            if input_first == input_name_short {
                                input_span_opt = Some(SourceSpan::from((
                                    cmd_execution_src.len(),
                                    input_first.len(),
                                )));
                            }
                            write!(&mut cmd_execution_src, "{input_first}")?;
                            input_remainder
                                .iter()
                                .try_for_each(|cmd_block_input_name| {
                                    if cmd_block_input_name == input_name_short {
                                        input_span_opt = Some(SourceSpan::from((
                                            // + 2 is for the comma and space
                                            cmd_execution_src.len() + 2,
                                            cmd_block_input_name.len(),
                                        )));
                                    }
                                    write!(&mut cmd_execution_src, ", {cmd_block_input_name}")?;
                                    Ok(())
                                })?;
                            writeln!(&mut cmd_execution_src, ")")?;
                        }
                    }
                }

                write!(&mut cmd_execution_src, "    Outcome: ")?;
                match cmd_block_desc.cmd_block_outcome_names().split_first() {
                    None => write!(&mut cmd_execution_src, "()")?,
                    Some((outcome_first, outcome_remainder)) => {
                        if outcome_remainder.is_empty() {
                            writeln!(&mut cmd_execution_src, "{outcome_first}")?;
                        } else {
                            write!(&mut cmd_execution_src, "(")?;
                            write!(&mut cmd_execution_src, "{outcome_first}")?;
                            outcome_remainder
                                .iter()
                                .try_for_each(|cmd_block_outcome_name| {
                                    write!(&mut cmd_execution_src, ", {cmd_block_outcome_name}")?;
                                    Ok(())
                                })?;
                            writeln!(&mut cmd_execution_src, ")")?;
                        }
                    }
                }

                Ok(input_span_opt)
            })?;

    Ok::<_, fmt::Error>((cmd_execution_src, input_span))
}
