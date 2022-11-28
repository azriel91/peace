//! Manages a synchronization shell command for the peace framework.

pub use crate::{
    sh_sync_cmd::ShSyncCmd, sh_sync_cmd_clean_op_spec::ShSyncCmdCleanOpSpec,
    sh_sync_cmd_data::ShSyncCmdData, sh_sync_cmd_ensure_op_spec::ShSyncCmdEnsureOpSpec,
    sh_sync_cmd_error::ShSyncCmdError, sh_sync_cmd_execution_record::ShSyncCmdExecutionRecord,
    sh_sync_cmd_item_spec::ShSyncCmdItemSpec, sh_sync_cmd_params::ShSyncCmdParams,
    sh_sync_cmd_state_current_fn_spec::ShSyncCmdStateCurrentFnSpec,
    sh_sync_cmd_state_desired_fn_spec::ShSyncCmdStateDesiredFnSpec,
    sh_sync_cmd_state_diff::ShSyncCmdStateDiff,
    sh_sync_cmd_state_diff_fn_spec::ShSyncCmdStateDiffFnSpec,
    sh_sync_cmd_sync_status::ShSyncCmdSyncStatus,
};

mod sh_sync_cmd;
mod sh_sync_cmd_clean_op_spec;
mod sh_sync_cmd_data;
mod sh_sync_cmd_ensure_op_spec;
mod sh_sync_cmd_error;
mod sh_sync_cmd_execution_record;
mod sh_sync_cmd_item_spec;
mod sh_sync_cmd_params;
mod sh_sync_cmd_state_current_fn_spec;
mod sh_sync_cmd_state_desired_fn_spec;
mod sh_sync_cmd_state_diff;
mod sh_sync_cmd_state_diff_fn_spec;
mod sh_sync_cmd_sync_status;
