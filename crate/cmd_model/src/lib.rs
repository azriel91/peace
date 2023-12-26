//! Data types for commands for the Peace framework.
//!
//! Currently contains types for better error messages.

// Re-exports
pub use fn_graph;
pub use indexmap;

pub use crate::{
    cmd_block_desc::CmdBlockDesc, cmd_block_outcome::CmdBlockOutcome,
    cmd_execution_error::CmdExecutionError, cmd_outcome::CmdOutcome,
};

mod cmd_block_desc;
mod cmd_block_outcome;
mod cmd_execution_error;
mod cmd_outcome;
