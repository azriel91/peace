use fn_graph::StreamOutcome;
use futures::Future;
use indexmap::IndexMap;
use peace_cfg::ItemId;

use crate::CmdBlockDesc;

#[derive(Clone, Debug)]
pub enum CmdOutcome<T, E> {
    /// Execution completed successfully.
    Complete {
        /// The outcome value.
        value: T,
    },
    /// Execution ended due to an interruption during command block execution.
    BlockInterrupted {
        /// The stream outcome of the interrupted command block.
        stream_outcome: StreamOutcome<T>,
    },
    /// Execution ended due to an interruption between command blocks.
    ExecutionInterrupted {
        /// The outcome value.
        value: Option<T>,
        /// Descriptors of the `CmdBlock`s that were processed.
        cmd_blocks_processed: Vec<CmdBlockDesc>,
        /// Descriptors of the `CmdBlock`s that were not processed.
        cmd_blocks_not_processed: Vec<CmdBlockDesc>,
    },
    /// Execution ended due to one or more item errors.
    ///
    /// It is also possible for the stream to be interrupted when an error
    /// occurs, so the value is wrapped in a `StreamOutcome`.
    ItemError {
        /// The outcome value.
        stream_outcome: StreamOutcome<T>,
        /// Item error(s) from the last command block's execution.
        errors: IndexMap<ItemId, E>,
    },
}

impl<T, E> CmdOutcome<T, E> {
    pub fn value(&self) -> Option<&T> {
        match self {
            CmdOutcome::Complete { value } => Some(value),
            CmdOutcome::BlockInterrupted { stream_outcome } => Some(stream_outcome.value()),
            CmdOutcome::ExecutionInterrupted { value, .. } => value.as_ref(),
            CmdOutcome::ItemError {
                stream_outcome,
                errors: _,
            } => Some(stream_outcome.value()),
        }
    }

    /// Returns whether the command completed successfully.
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete { .. })
    }

    /// Returns whether the command completed successfully.
    pub fn is_interrupted(&self) -> bool {
        matches!(
            self,
            Self::BlockInterrupted { .. } | Self::ExecutionInterrupted { .. }
        )
    }

    /// Returns whether the command encountered item errors during execution.
    pub fn is_err(&self) -> bool {
        matches!(self, Self::ItemError { .. })
    }

    /// Maps the inner value to another, maintaining any collected errors.
    pub fn map<F, U>(self, f: F) -> CmdOutcome<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Complete { value: t } => {
                let u = f(t);
                CmdOutcome::Complete { value: u }
            }
            Self::BlockInterrupted { stream_outcome } => {
                let stream_outcome = stream_outcome.map(f);
                CmdOutcome::BlockInterrupted { stream_outcome }
            }
            Self::ExecutionInterrupted {
                value: t,
                cmd_blocks_processed,
                cmd_blocks_not_processed,
            } => {
                let u = t.map(f);
                CmdOutcome::ExecutionInterrupted {
                    value: u,
                    cmd_blocks_processed,
                    cmd_blocks_not_processed,
                }
            }
            Self::ItemError {
                stream_outcome,
                errors,
            } => {
                let stream_outcome = stream_outcome.map(f);
                CmdOutcome::ItemError {
                    stream_outcome,
                    errors,
                }
            }
        }
    }

    /// Maps the inner value to another asynchronously, maintaining any
    /// collected errors.
    pub async fn map_async<'f, F, Fut, U>(self, f: F) -> CmdOutcome<U, E>
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = U> + 'f,
    {
        match self {
            Self::Complete { value: t } => {
                let u = f(t).await;
                CmdOutcome::Complete { value: u }
            }
            Self::BlockInterrupted { stream_outcome } => {
                let (stream_outcome, value) = stream_outcome.replace(());
                let value = f(value).await;
                let (stream_outcome, ()) = stream_outcome.replace(value);
                CmdOutcome::BlockInterrupted { stream_outcome }
            }
            Self::ExecutionInterrupted {
                value: t,
                cmd_blocks_processed,
                cmd_blocks_not_processed,
            } => {
                let u = match t {
                    Some(t) => Some(f(t).await),
                    None => None,
                };
                CmdOutcome::ExecutionInterrupted {
                    value: u,
                    cmd_blocks_processed,
                    cmd_blocks_not_processed,
                }
            }
            Self::ItemError {
                stream_outcome,
                errors,
            } => {
                let (stream_outcome, value) = stream_outcome.replace(());
                let value = f(value).await;
                let (stream_outcome, ()) = stream_outcome.replace(value);
                CmdOutcome::ItemError {
                    stream_outcome,
                    errors,
                }
            }
        }
    }
}

impl<T, E> CmdOutcome<Result<T, E>, E> {
    /// Transposes a `CmdOutcome<Result<T, E>, E>` to a `Result<CmdOutcome<T,
    /// E>, E>`.
    pub fn transpose(self) -> Result<CmdOutcome<T, E>, E> {
        match self {
            Self::Complete { value } => match value {
                Ok(value) => Ok(CmdOutcome::Complete { value }),
                Err(e) => Err(e),
            },
            Self::BlockInterrupted { stream_outcome } => {
                let (stream_outcome, value) = stream_outcome.replace(());
                match value {
                    Ok(value) => {
                        let (stream_outcome, ()) = stream_outcome.replace(value);
                        Ok(CmdOutcome::BlockInterrupted { stream_outcome })
                    }
                    Err(e) => Err(e),
                }
            }
            Self::ExecutionInterrupted {
                value,
                cmd_blocks_processed,
                cmd_blocks_not_processed,
            } => match value.transpose() {
                Ok(value) => Ok(CmdOutcome::ExecutionInterrupted {
                    value,
                    cmd_blocks_processed,
                    cmd_blocks_not_processed,
                }),
                Err(e) => Err(e),
            },
            Self::ItemError {
                stream_outcome,
                errors,
            } => {
                let (stream_outcome, value) = stream_outcome.replace(());
                match value {
                    Ok(value) => {
                        let (stream_outcome, ()) = stream_outcome.replace(value);
                        Ok(CmdOutcome::ItemError {
                            stream_outcome,
                            errors,
                        })
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }
}
