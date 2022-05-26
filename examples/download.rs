use std::path::PathBuf;

pub use crate::{
    download_clean_spec::DownloadCleanSpec,
    download_ensure_spec::DownloadEnsureSpec,
    download_error::DownloadError,
    download_full_spec::DownloadFullSpec,
    download_params::DownloadParams,
    download_status_spec::DownloadStatusSpec,
    file_state::{FileState, FileStateDiff},
};

#[path = "download/download_clean_spec.rs"]
mod download_clean_spec;
#[path = "download/download_ensure_spec.rs"]
mod download_ensure_spec;
#[path = "download/download_error.rs"]
mod download_error;
#[path = "download/download_full_spec.rs"]
mod download_full_spec;
#[path = "download/download_params.rs"]
mod download_params;
#[path = "download/download_status_spec.rs"]
mod download_status_spec;
#[path = "download/file_state.rs"]
mod file_state;

fn main() {
    //
}

/// Read up to 1 kB in memory.
pub const IN_MEMORY_CONTENTS_MAX: u64 = 1024;
