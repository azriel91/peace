use clap::Parser;
use peace::{
    cfg::{profile, Profile},
    rt_model::WorkspaceSpec,
};
use tokio::io;

pub use download::{
    desired, diff, ensure, ensure_dry, setup_workspace, status, DownloadArgs, DownloadCleanOpSpec,
    DownloadCommand, DownloadEnsureOpSpec, DownloadError, DownloadItemSpec, DownloadParams,
    DownloadStateCurrentFnSpec, DownloadStateDesiredFnSpec, DownloadStateDiffFnSpec, FileState,
    FileStateDiff,
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
        let workspace_spec = &WorkspaceSpec::WorkingDir;
        let profile = profile!("default");

        match command {
            DownloadCommand::Status { url, dest } => {
                let workspace = setup_workspace(workspace_spec, profile, url, dest).await?;
                status(io::stdout(), workspace).await?;
            }
            DownloadCommand::Desired { url, dest } => {
                let workspace = setup_workspace(workspace_spec, profile, url, dest).await?;
                desired(io::stdout(), workspace).await?;
            }
            DownloadCommand::Diff { url, dest } => {
                let workspace = setup_workspace(workspace_spec, profile, url, dest).await?;
                diff(io::stdout(), workspace).await?;
            }
            DownloadCommand::EnsureDry { url, dest } => {
                let workspace = setup_workspace(workspace_spec, profile, url, dest).await?;
                ensure_dry(io::stdout(), workspace).await?;
            }
            DownloadCommand::Ensure { url, dest } => {
                let workspace = setup_workspace(workspace_spec, profile, url, dest).await?;
                ensure(io::stdout(), workspace).await?;
            }
        }

        Ok::<_, DownloadError>(())
    })
}
