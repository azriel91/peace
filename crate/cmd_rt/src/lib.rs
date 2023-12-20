//! Runtime types for commands for the Peace framework.

// Re-exports
pub use async_trait::async_trait;
pub use tynm;

pub use crate::{
    cmd_block::{CmdBlock, CmdBlockError, CmdBlockRt, CmdBlockRtBox, CmdBlockWrapper},
    cmd_execution::{CmdExecution, CmdExecutionBuilder},
};

mod cmd_block;
mod cmd_execution;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        /// Maximum number of progress messages to buffer.
        ///
        /// TODO: Remove the copy in `peace_rt`.
        pub const PROGRESS_COUNT_MAX: usize = 256;

        pub(crate) use crate::progress::Progress;
        pub(crate) mod progress;
    }
}
