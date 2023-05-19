//! Manages running a shell command for the peace framework.
//!
//! This item spec is designed to take in separate shell commands for each of
//! the following:
//!
//! * Current state logic, whose stdout defines the current state (`String`).
//! * Desired state logic, whose stdout defines the desired state (`String`).
//! * State diff logic, whose stdout defines the state difference.
//! * Ensure check, whose stdout defines if ensure execution needs to run --
//!   `true` means execution is required, `false` means execution is required.
//! * Ensure execution, whose stdout defines state physical.
//! * Clean check, whose exit status defines if clean execution needs to run --
//!   `true` means execution is required, `false` means execution is required.
//! * Clean execution.

pub use crate::{
    cmd_variant::CmdVariant,
    sh_cmd::ShCmd,
    sh_cmd_apply_fns::ShCmdApplyFns,
    sh_cmd_data::ShCmdData,
    sh_cmd_error::ShCmdError,
    sh_cmd_execution_record::ShCmdExecutionRecord,
    sh_cmd_item_spec::ShCmdItemSpec,
    sh_cmd_params::{ShCmdParams, ShCmdParamsFieldWise, ShCmdParamsPartial},
    sh_cmd_state::ShCmdState,
    sh_cmd_state_diff::ShCmdStateDiff,
    sh_cmd_state_diff_fn::ShCmdStateDiffFn,
};

pub(crate) use sh_cmd_executor::ShCmdExecutor;

mod cmd_variant;
mod sh_cmd;
mod sh_cmd_apply_fns;
mod sh_cmd_data;
mod sh_cmd_error;
mod sh_cmd_execution_record;
mod sh_cmd_executor;
mod sh_cmd_item_spec;
mod sh_cmd_params;
mod sh_cmd_state;
mod sh_cmd_state_diff;
mod sh_cmd_state_diff_fn;
