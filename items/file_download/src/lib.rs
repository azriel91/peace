//! Manages downloading a file for the peace framework

pub use crate::{
    e_tag::ETag,
    file_download_apply_fns::FileDownloadApplyFns,
    file_download_data::FileDownloadData,
    file_download_error::FileDownloadError,
    file_download_item::FileDownloadItem,
    file_download_params::{
        FileDownloadParams, FileDownloadParamsFieldWise, FileDownloadParamsPartial,
    },
    file_download_state::FileDownloadState,
    file_download_state_current_fn::FileDownloadStateCurrentFn,
    file_download_state_diff::FileDownloadStateDiff,
    file_download_state_diff_fn::FileDownloadStateDiffFn,
    file_download_state_goal_fn::FileDownloadStateGoalFn,
    file_download_state_logical::FileDownloadStateLogical,
};

#[cfg(target_arch = "wasm32")]
pub use crate::storage_form::StorageForm;

mod e_tag;
mod file_download_apply_fns;
mod file_download_data;
mod file_download_error;
mod file_download_item;
mod file_download_params;
mod file_download_state;
mod file_download_state_current_fn;
mod file_download_state_diff;
mod file_download_state_diff_fn;
mod file_download_state_goal_fn;
mod file_download_state_logical;

#[cfg(target_arch = "wasm32")]
mod storage_form;

/// Read up to 1 kB in memory.
pub const IN_MEMORY_CONTENTS_MAX: u64 = 1024;
