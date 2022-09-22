use clap::Parser;
use peace::{
    cfg::{flow_id, profile, FlowId, Profile},
    rt_model::{CliOutput, WorkspaceSpec},
};

use download::{
    cmd_context, desired, diff, ensure, ensure_dry, fetch, status, workspace_and_graph_setup,
    workspace_init, DownloadArgs, DownloadCommand, DownloadError,
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
        let flow_id = flow_id!("file");
        let mut cli_output = CliOutput::default();

        match command {
            DownloadCommand::Init { url, dest } => {
                let workspace_and_graph =
                    workspace_init(workspace_spec, profile, flow_id, url, dest).await?;
                let cmd_context = cmd_context(&workspace_and_graph, &mut cli_output).await?;
                fetch(cmd_context).await?;
            }
            DownloadCommand::Fetch => {
                let workspace_and_graph =
                    workspace_and_graph_setup(workspace_spec, profile, flow_id).await?;
                let cmd_context = cmd_context(&workspace_and_graph, &mut cli_output).await?;
                fetch(cmd_context).await?;
            }
            DownloadCommand::Status => {
                let workspace_and_graph =
                    workspace_and_graph_setup(workspace_spec, profile, flow_id).await?;
                let cmd_context = cmd_context(&workspace_and_graph, &mut cli_output).await?;
                status(cmd_context).await?;
            }
            DownloadCommand::Desired => {
                let workspace_and_graph =
                    workspace_and_graph_setup(workspace_spec, profile, flow_id).await?;
                let cmd_context = cmd_context(&workspace_and_graph, &mut cli_output).await?;
                desired(cmd_context).await?;
            }
            DownloadCommand::Diff => {
                let workspace_and_graph =
                    workspace_and_graph_setup(workspace_spec, profile, flow_id).await?;
                let cmd_context = cmd_context(&workspace_and_graph, &mut cli_output).await?;
                diff(cmd_context).await?;
            }
            DownloadCommand::EnsureDry => {
                let workspace_and_graph =
                    workspace_and_graph_setup(workspace_spec, profile, flow_id).await?;
                let cmd_context = cmd_context(&workspace_and_graph, &mut cli_output).await?;
                ensure_dry(cmd_context).await?;
            }
            DownloadCommand::Ensure => {
                let workspace_and_graph =
                    workspace_and_graph_setup(workspace_spec, profile, flow_id).await?;
                let cmd_context = cmd_context(&workspace_and_graph, &mut cli_output).await?;
                ensure(cmd_context).await?;
            }
        }

        Ok::<_, DownloadError>(())
    })
}
