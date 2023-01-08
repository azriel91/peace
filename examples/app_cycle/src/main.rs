use app_cycle::{
    cmds::AppInitCmd,
    model::{
        cli_args::{AppCycleCommand, CliArgs, ProfileCommand},
        AppCycleError,
    },
};
use clap::Parser;
use peace::{
    cfg::{flow_id, profile, FlowId, Profile},
    rt_model::{output::CliOutput, WorkspaceSpec},
};

#[cfg(not(feature = "error_reporting"))]
pub fn main() -> Result<(), AppCycleError> {
    run()
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

    run().map_err(|app_cycle_error| match app_cycle_error {
        AppCycleError::PeaceItemSpecFileDownload(err) => peace::miette::Report::from(err),
        AppCycleError::PeaceRtError(err) => peace::miette::Report::from(err),
        other => peace::miette::Report::from(other),
    })
}

pub fn run() -> Result<(), AppCycleError> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .thread_name("main")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_io()
        .enable_time()
        .build()
        .map_err(AppCycleError::TokioRuntimeInit)?;

    let CliArgs { command, format } = CliArgs::parse();
    #[allow(unused_assignments)]
    runtime.block_on(async {
        let _workspace_spec = WorkspaceSpec::WorkingDir;
        let _profile = profile!("default");
        let _flow_id = flow_id!("file");
        let mut cli_output = CliOutput::default();
        if let Some(format) = format {
            cli_output = cli_output.with_output_format(format);
        }

        match command {
            AppCycleCommand::Init { slug, version } => {
                AppInitCmd::run(&mut cli_output, slug, version).await?
            }
            AppCycleCommand::Profile { command } => match command {
                ProfileCommand::Init { name: _, r#type: _ } => todo!(),
                ProfileCommand::List => todo!(),
                ProfileCommand::Show => todo!(),
            },
            AppCycleCommand::Switch { profile: _ } => todo!(),
            AppCycleCommand::Fetch => todo!(),
            AppCycleCommand::Status => todo!(),
            AppCycleCommand::Desired => todo!(),
            AppCycleCommand::Diff => todo!(),
            AppCycleCommand::Push => todo!(),
            AppCycleCommand::Pull => todo!(),
            AppCycleCommand::Clean => todo!(),
        }

        Ok::<_, AppCycleError>(())
    })
}
