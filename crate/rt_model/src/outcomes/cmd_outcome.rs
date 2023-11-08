use peace_cfg::ItemId;
use peace_data::fn_graph::StreamOutcome;
use peace_rt_model_core::IndexMap;

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
        value: T,
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
            Self::ExecutionInterrupted { value: t } => {
                let u = f(t);
                CmdOutcome::ExecutionInterrupted { value: u }
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
}
