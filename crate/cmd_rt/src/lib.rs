//! Runtime types for commands for the Peace framework.

// Re-exports
pub use async_trait::async_trait;
pub use tynm;

pub use crate::{
    cmd_block::{CmdBlock, CmdBlockError, CmdBlockRt, CmdBlockRtBox, CmdBlockWrapper},
    cmd_execution::{CmdExecution, CmdExecutionBuilder},
    step_stream_outcome_mapper::StepStreamOutcomeMapper,
};

mod cmd_block;
mod cmd_execution;
mod step_stream_outcome_mapper;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        /// Maximum number of progress messages to buffer.
        pub const CMD_PROGRESS_COUNT_MAX: usize = 256;

        pub(crate) use crate::progress::Progress;
        pub(crate) mod progress;
    }
}
