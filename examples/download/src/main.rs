use clap::Parser;
use peace::{
    cfg::{flow_id, profile, FlowId, Profile},
    rt_model::{output::CliOutput, WorkspaceSpec},
};
use peace_item_specs::file_download::FileDownloadParams;

use download::{
    clean, clean_dry, cmd_context, desired, diff, ensure, ensure_dry, fetch, status,
    workspace_and_graph_setup, DownloadArgs, DownloadCommand, DownloadError,
};

#[cfg(not(feature = "error_reporting"))]
pub fn main() {
    run().unwrap();
}

#[cfg(feature = "error_reporting")]
pub fn main() -> peace::miette::Result<(), peace::miette::Report> {
    // Important to return `peace::miette::Report` instead of calling
    // `IntoDiagnostic::intoDiagnostic` on the `Error`, as that does not present the
    // diagnostic contextual information to the user.
    //
    // See <https://docs.rs/miette/latest/miette/trait.IntoDiagnostic.html#warning>.

    // The explicit mapping for `PeaceRtError` appears to be necessary to display
    // the diagnostic information. i.e. `miette` does not automatically delegate to
    // the #[diagnostic_source].
    //
    // This is fixed by <https://github.com/zkat/miette/pull/170>.

    run().map_err(|file_download_error| match file_download_error {
        DownloadError::PeaceItemSpecFileDownload(err) => peace::miette::Report::from(err),
        DownloadError::PeaceRtError(err) => peace::miette::Report::from(err),
        other => peace::miette::Report::from(other),
    })
}

pub fn run() -> Result<(), DownloadError> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .thread_name("main")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_io()
        .enable_time()
        .build()
        .map_err(DownloadError::TokioRuntimeInit)?;

    let DownloadArgs {
        command,
        verbose,
        format,
    } = DownloadArgs::parse();

    if !verbose {
        #[cfg(feature = "error_reporting")]
        peace::miette::set_hook(Box::new(|_| {
            Box::new(
                peace::miette::MietteHandlerOpts::new()
                    .without_cause_chain()
                    .build(),
            )
        }))
        .expect("Failed to configure miette hook.");
    }

    runtime.block_on(async {
        let workspace_spec = WorkspaceSpec::WorkingDir;
        let profile = profile!("default");
        let flow_id = flow_id!("file");
        let mut cli_output = {
            let mut builder = CliOutput::builder();
            if let Some(format) = format {
                builder = builder.with_outcome_format(format);
            }
            builder.build()
        };

        match command {
            DownloadCommand::Init { url, dest } => {
                let workspace_and_graph = workspace_and_graph_setup(workspace_spec).await?;
                let cmd_context = cmd_context(
                    &workspace_and_graph,
                    profile,
                    flow_id,
                    &mut cli_output,
                    Some(FileDownloadParams::new(url, dest)),
                )
                .await?;
                fetch(cmd_context).await?;
            }
            DownloadCommand::Fetch => {
                let workspace_and_graph = workspace_and_graph_setup(workspace_spec).await?;
                let cmd_context = cmd_context(
                    &workspace_and_graph,
                    profile,
                    flow_id,
                    &mut cli_output,
                    None,
                )
                .await?;
                fetch(cmd_context).await?;
            }
            DownloadCommand::Status => {
                let workspace_and_graph = workspace_and_graph_setup(workspace_spec).await?;
                let cmd_context = cmd_context(
                    &workspace_and_graph,
                    profile,
                    flow_id,
                    &mut cli_output,
                    None,
                )
                .await?;
                status(cmd_context).await?;
            }
            DownloadCommand::Desired => {
                let workspace_and_graph = workspace_and_graph_setup(workspace_spec).await?;
                let cmd_context = cmd_context(
                    &workspace_and_graph,
                    profile,
                    flow_id,
                    &mut cli_output,
                    None,
                )
                .await?;
                desired(cmd_context).await?;
            }
            DownloadCommand::Diff => {
                let workspace_and_graph = workspace_and_graph_setup(workspace_spec).await?;
                let cmd_context = cmd_context(
                    &workspace_and_graph,
                    profile,
                    flow_id,
                    &mut cli_output,
                    None,
                )
                .await?;
                diff(cmd_context).await?;
            }
            DownloadCommand::EnsureDry => {
                let workspace_and_graph = workspace_and_graph_setup(workspace_spec).await?;
                let cmd_context = cmd_context(
                    &workspace_and_graph,
                    profile,
                    flow_id,
                    &mut cli_output,
                    None,
                )
                .await?;
                ensure_dry(cmd_context).await?;
            }
            DownloadCommand::Ensure => {
                let workspace_and_graph = workspace_and_graph_setup(workspace_spec).await?;
                let cmd_context = cmd_context(
                    &workspace_and_graph,
                    profile,
                    flow_id,
                    &mut cli_output,
                    None,
                )
                .await?;
                ensure(cmd_context).await?;
            }
            DownloadCommand::CleanDry => {
                let workspace_and_graph = workspace_and_graph_setup(workspace_spec).await?;
                let cmd_context = cmd_context(
                    &workspace_and_graph,
                    profile,
                    flow_id,
                    &mut cli_output,
                    None,
                )
                .await?;
                clean_dry(cmd_context).await?;
            }
            DownloadCommand::Clean => {
                let workspace_and_graph = workspace_and_graph_setup(workspace_spec).await?;
                let cmd_context = cmd_context(
                    &workspace_and_graph,
                    profile,
                    flow_id,
                    &mut cli_output,
                    None,
                )
                .await?;
                clean(cmd_context).await?;
            }
        }

        Ok::<_, DownloadError>(())
    })
}
