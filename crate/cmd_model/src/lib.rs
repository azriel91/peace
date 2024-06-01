//! Data types for commands for the Peace framework.
//!
//! Currently contains types for better error messages.

// Re-exports
pub use fn_graph;
pub use indexmap;

pub use crate::{
    cmd_block_desc::CmdBlockDesc, cmd_block_outcome::CmdBlockOutcome,
    cmd_execution_error::CmdExecutionError, cmd_execution_id::CmdExecutionId,
    cmd_outcome::CmdOutcome, step_stream_outcome::StepStreamOutcome,
    stream_outcome_and_errors::StreamOutcomeAndErrors,
    value_and_stream_outcome::ValueAndStreamOutcome,
};

mod cmd_block_desc;
mod cmd_block_outcome;
mod cmd_execution_error;
mod cmd_execution_id;
mod cmd_outcome;
mod step_stream_outcome;
mod stream_outcome_and_errors;
mod value_and_stream_outcome;
