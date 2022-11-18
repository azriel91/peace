//! Manages running a shell command for the peace framework

pub use crate::{
    sh_cmd_clean_op_spec::ShCmdCleanOpSpec, sh_cmd_data::ShCmdData,
    sh_cmd_ensure_op_spec::ShCmdEnsureOpSpec, sh_cmd_error::ShCmdError,
    sh_cmd_item_spec::ShCmdItemSpec, sh_cmd_params::ShCmdParams, sh_cmd_state::ShCmdState,
    sh_cmd_state_current_fn_spec::ShCmdStateCurrentFnSpec,
    sh_cmd_state_desired_fn_spec::ShCmdStateDesiredFnSpec, sh_cmd_state_diff::ShCmdStateDiff,
    sh_cmd_state_diff_fn_spec::ShCmdStateDiffFnSpec,
};

mod sh_cmd_clean_op_spec;
mod sh_cmd_data;
mod sh_cmd_ensure_op_spec;
mod sh_cmd_error;
mod sh_cmd_item_spec;
mod sh_cmd_params;
mod sh_cmd_state;
mod sh_cmd_state_current_fn_spec;
mod sh_cmd_state_desired_fn_spec;
mod sh_cmd_state_diff;
mod sh_cmd_state_diff_fn_spec;
