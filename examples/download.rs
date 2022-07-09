use std::io;

use futures::{StreamExt, TryStreamExt};
use peace::{
    data::Resources,
    rt::StatusCommand,
    rt_model::{self, FullSpecGraphBuilder, FullSpecResourceses},
};
use tokio::runtime::Builder;

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
        let mut resources = Resources::new();

        let mut graph_builder = FullSpecGraphBuilder::<DownloadError>::new();
        graph_builder.add_fn(DownloadFullSpec.into());

        let graph = graph_builder.build();

        // TODO: put this in the framework
        let mut full_spec_resourceses = FullSpecResourceses::new();
        (0..graph.node_count()).for_each(|_| full_spec_resourceses.push(Resources::new()));
        resources.insert(full_spec_resourceses);

        // setup resources
        let resources = graph
            .stream()
            .map(Ok::<_, rt_model::Error<DownloadError>>)
            .try_fold(resources, |mut resources, full_spec| async move {
                full_spec.setup(&mut resources).await.unwrap();
                Ok(resources)
            })
            .await
            .unwrap();

        StatusCommand::exec(&graph, &resources).await.unwrap();

        resources.iter().for_each(|resource| {
            println!("{resource:?}");
        });
    });

    Ok(())
}

/// Read up to 1 kB in memory.
pub const IN_MEMORY_CONTENTS_MAX: u64 = 1024;
