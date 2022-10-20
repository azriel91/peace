use clap::Parser;
use peace::{
    cfg::{flow_id, profile, FlowId, Profile},
    rt_model::{CliOutput, WorkspaceSpec},
};

use download::{
    clean, clean_dry, cmd_context, desired, diff, ensure, ensure_dry, fetch, status,
    workspace_and_graph_setup, DownloadArgs, DownloadCommand, DownloadError, DownloadProfileInit,
};

#[cfg(not(feature = "error_reporting"))]
pub fn main() -> Result<(), DownloadError> {
    run()
}

#[cfg(feature = "error_reporting")]
pub fn main() -> peace::miette::Result<(), peace::miette::Report> {
    // Important to return `peace::miette::Report` instead of calling
    // `IntoDiagnostic::intoDiagnostic` on the `Error`, as that does not present the
    // diagnostic contextual information to the user.
    //
    // See <https://docs.rs/miette/latest/miette/trait.IntoDiagnostic.html#warning>.
    run().map_err(Into::into)
}

pub fn run() -> Result<(), DownloadError> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .thread_name("main")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_io()
        .enable_time()
        .build()
        .map_err(DownloadError::TokioRuntimeInit)?;

    let DownloadArgs { command, format } = DownloadArgs::parse();
    runtime.block_on(async {
        let workspace_spec = WorkspaceSpec::WorkingDir;
        let profile = profile!("default");
        let flow_id = flow_id!("file");
        let mut cli_output = CliOutput::default();
        if let Some(format) = format {
            cli_output = cli_output.output_format(format);
        }
        #[cfg(feature = "output_colorized")]
        {
            cli_output = cli_output.colorized();
        }

        match command {
            DownloadCommand::Init { url, dest } => {
                let workspace_and_graph =
                    workspace_and_graph_setup(workspace_spec, profile, flow_id).await?;
                let cmd_context = cmd_context(
                    &workspace_and_graph,
                    &mut cli_output,
                    Some(DownloadProfileInit::new(url, dest)),
                )
                .await?;
                fetch(cmd_context).await?;
            }
            DownloadCommand::Fetch => {
                let workspace_and_graph =
                    workspace_and_graph_setup(workspace_spec, profile, flow_id).await?;
                let cmd_context = cmd_context(&workspace_and_graph, &mut cli_output, None).await?;
                fetch(cmd_context).await?;
            }
            DownloadCommand::Status => {
                let workspace_and_graph =
                    workspace_and_graph_setup(workspace_spec, profile, flow_id).await?;
                let cmd_context = cmd_context(&workspace_and_graph, &mut cli_output, None).await?;
                status(cmd_context).await?;
            }
            DownloadCommand::Desired => {
                let workspace_and_graph =
                    workspace_and_graph_setup(workspace_spec, profile, flow_id).await?;
                let cmd_context = cmd_context(&workspace_and_graph, &mut cli_output, None).await?;
                desired(cmd_context).await?;
            }
            DownloadCommand::Diff => {
                let workspace_and_graph =
                    workspace_and_graph_setup(workspace_spec, profile, flow_id).await?;
                let cmd_context = cmd_context(&workspace_and_graph, &mut cli_output, None).await?;
                diff(cmd_context).await?;
            }
            DownloadCommand::EnsureDry => {
                let workspace_and_graph =
                    workspace_and_graph_setup(workspace_spec, profile, flow_id).await?;
                let cmd_context = cmd_context(&workspace_and_graph, &mut cli_output, None).await?;
                ensure_dry(cmd_context).await?;
            }
            DownloadCommand::Ensure => {
                let workspace_and_graph =
                    workspace_and_graph_setup(workspace_spec, profile, flow_id).await?;
                let cmd_context = cmd_context(&workspace_and_graph, &mut cli_output, None).await?;
                ensure(cmd_context).await?;
            }
            DownloadCommand::CleanDry => {
                let workspace_and_graph =
                    workspace_and_graph_setup(workspace_spec, profile, flow_id).await?;
                let cmd_context = cmd_context(&workspace_and_graph, &mut cli_output, None).await?;
                clean_dry(cmd_context).await?;
            }
            DownloadCommand::Clean => {
                let workspace_and_graph =
                    workspace_and_graph_setup(workspace_spec, profile, flow_id).await?;
                let cmd_context = cmd_context(&workspace_and_graph, &mut cli_output, None).await?;
                clean(cmd_context).await?;
            }
        }

        Ok::<_, DownloadError>(())
    })
}
