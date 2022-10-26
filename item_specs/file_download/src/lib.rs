//! Manages downloading a file for the peace framework

pub use crate::{
    file_download_clean_op_spec::FileDownloadCleanOpSpec,
    file_download_ensure_op_spec::FileDownloadEnsureOpSpec, file_download_error::FileDownloadError,
    file_download_item_spec::FileDownloadItemSpec, file_download_params::FileDownloadParams,
    file_download_profile_init::FileDownloadProfileInit, file_download_state::FileDownloadState,
    file_download_state_current_fn_spec::FileDownloadStateCurrentFnSpec,
    file_download_state_desired_fn_spec::FileDownloadStateDesiredFnSpec,
    file_download_state_diff::FileDownloadStateDiff,
    file_download_state_diff_fn_spec::FileDownloadStateDiffFnSpec,
};

mod file_download_clean_op_spec;
mod file_download_ensure_op_spec;
mod file_download_error;
mod file_download_item_spec;
mod file_download_params;
mod file_download_profile_init;
mod file_download_state;
mod file_download_state_current_fn_spec;
mod file_download_state_desired_fn_spec;
mod file_download_state_diff;
mod file_download_state_diff_fn_spec;

/// Read up to 1 kB in memory.
pub const IN_MEMORY_CONTENTS_MAX: u64 = 1024;
