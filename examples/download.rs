use std::{io, path::Path};

use peace::{resources::Resources, rt::StatusCommand, rt_model::FullSpecGraphBuilder};
use tokio::runtime::Builder;
use url::Url;

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

fn main() -> io::Result<()> {
    let runtime = Builder::new_current_thread()
        .thread_name("main")
        .thread_stack_size(3 * 1024 * 1024)
        .build()?;

    runtime.block_on(async {
        let url =
            Url::parse("https://ifconfig.me/all.json").expect("Expected download URL to be valid.");
        let dest = Path::new("all.json").to_path_buf();

        let mut graph_builder = FullSpecGraphBuilder::<DownloadError>::new();
        graph_builder.add_fn(DownloadFullSpec::new(url, dest).into());

        let graph = graph_builder.build();

        let resources = graph.setup(Resources::new()).await.unwrap();

        StatusCommand::exec(&graph, &resources).await.unwrap();

        resources.iter().for_each(|resource| {
            println!("{resource:#?}");
        });
    });

    Ok(())
}

/// Read up to 1 kB in memory.
pub const IN_MEMORY_CONTENTS_MAX: u64 = 1024;
