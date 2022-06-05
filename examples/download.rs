use peace::rt_model::FullSpecGraphBuilder;

pub use crate::{
    download_clean_op_spec::DownloadCleanOpSpec,
    download_ensure_op_spec::DownloadEnsureOpSpec,
    download_error::DownloadError,
    download_full_spec::DownloadFullSpec,
    download_params::DownloadParams,
    download_status_fn_spec::DownloadStatusFnSpec,
    file_state::{FileState, FileStateDiff},
};

#[path = "download/download_clean_op_spec.rs"]
mod download_clean_op_spec;
#[path = "download/download_ensure_op_spec.rs"]
mod download_ensure_op_spec;
#[path = "download/download_error.rs"]
mod download_error;
#[path = "download/download_full_spec.rs"]
mod download_full_spec;
#[path = "download/download_params.rs"]
mod download_params;
#[path = "download/download_status_fn_spec.rs"]
mod download_status_fn_spec;
#[path = "download/file_state.rs"]
mod file_state;

fn main() {
    let mut graph_builder = FullSpecGraphBuilder::<DownloadError>::new();
    graph_builder.add_fn(DownloadFullSpec::default().into());

    let _graph = graph_builder.build();
}

/// Read up to 1 kB in memory.
pub const IN_MEMORY_CONTENTS_MAX: u64 = 1024;
