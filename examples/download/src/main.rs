use clap::Parser;
use peace::{
    cfg::{profile, Profile},
    rt_model::WorkspaceSpec,
};
use tokio::io;

pub use download::{
    cmd_context, desired, diff, ensure, ensure_dry, setup_workspace_and_graph, status,
    DownloadArgs, DownloadCleanOpSpec, DownloadCommand, DownloadEnsureOpSpec, DownloadError,
    DownloadItemSpec, DownloadParams, DownloadStateCurrentFnSpec, DownloadStateDesiredFnSpec,
    DownloadStateDiffFnSpec, FileState, FileStateDiff,
};

pub fn main() -> Result<(), DownloadError> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .thread_name("main")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_io()
        .enable_time()
        .build()
        .map_err(DownloadError::TokioRuntimeInit)?;

    let DownloadArgs { command } = DownloadArgs::parse();
    runtime.block_on(async {
        let workspace_spec = WorkspaceSpec::WorkingDir;
        let profile = profile!("default");

        match command {
            DownloadCommand::Status { url, dest } => {
                let workspace_and_graph =
                    setup_workspace_and_graph(workspace_spec, profile, url, dest).await?;
                let cmd_context = cmd_context(&workspace_and_graph).await?;
                status(io::stdout(), cmd_context).await?;
            }
            DownloadCommand::Desired { url, dest } => {
                let workspace_and_graph =
                    setup_workspace_and_graph(workspace_spec, profile, url, dest).await?;
                let cmd_context = cmd_context(&workspace_and_graph).await?;
                desired(io::stdout(), cmd_context).await?;
            }
            DownloadCommand::Diff { url, dest } => {
                let workspace_and_graph =
                    setup_workspace_and_graph(workspace_spec, profile, url, dest).await?;
                let cmd_context = cmd_context(&workspace_and_graph).await?;
                diff(io::stdout(), cmd_context).await?;
            }
            DownloadCommand::EnsureDry { url, dest } => {
                let workspace_and_graph =
                    setup_workspace_and_graph(workspace_spec, profile, url, dest).await?;
                let cmd_context = cmd_context(&workspace_and_graph).await?;
                ensure_dry(io::stdout(), cmd_context).await?;
            }
            DownloadCommand::Ensure { url, dest } => {
                let workspace_and_graph =
                    setup_workspace_and_graph(workspace_spec, profile, url, dest).await?;
                let cmd_context = cmd_context(&workspace_and_graph).await?;
                ensure(io::stdout(), cmd_context).await?;
            }
        }

        Ok::<_, DownloadError>(())
    })
}
