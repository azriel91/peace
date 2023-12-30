//! Runtime types for commands for the Peace framework.

// Re-exports
pub use async_trait::async_trait;
pub use tynm;

pub use crate::{
    cmd_block::{CmdBlock, CmdBlockError, CmdBlockRt, CmdBlockRtBox, CmdBlockWrapper},
    cmd_execution::{CmdExecution, CmdExecutionBuilder},
    item_stream_outcome_mapper::ItemStreamOutcomeMapper,
};

mod cmd_block;
mod cmd_execution;
mod item_stream_outcome_mapper;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        /// Maximum number of progress messages to buffer.
        ///
        /// TODO: Remove the copy in `peace_rt`.
        pub const PROGRESS_COUNT_MAX: usize = 256;
        /// Maximum number of `CmdExecution` progress messages to buffer.
        ///
        /// We don't expect many of these to happen at the same time, so a small limit is sufficient.
        pub const CMD_PROGRESS_COUNT_MAX: usize = 32;

        pub(crate) use crate::progress::Progress;
        pub(crate) mod progress;
    }
}
