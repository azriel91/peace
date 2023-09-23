//! Data types for commands for the Peace framework.
//!
//! Currently contains types for better error messages.

pub use crate::{cmd_block_desc::CmdBlockDesc, cmd_execution_error::CmdExecutionError};

mod cmd_block_desc;
mod cmd_execution_error;
