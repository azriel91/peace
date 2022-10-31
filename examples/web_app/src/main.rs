use clap::Parser;
use peace::{
    cfg::{flow_id, profile, FlowId, Profile},
    rt_model::{CliOutput, WorkspaceSpec},
};
use web_app::{
    cmds::AppInitCmd,
    model::{
        cli_args::{CliArgs, ProfileCommand, WebAppCommand},
        WebAppError,
    },
};

#[cfg(not(feature = "error_reporting"))]
pub fn main() -> Result<(), WebAppError> {
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

    run().map_err(|web_app_error| match web_app_error {
        WebAppError::PeaceItemSpecFileDownload(err) => peace::miette::Report::from(err),
        WebAppError::PeaceRtError(err) => peace::miette::Report::from(err),
        other => peace::miette::Report::from(other),
    })
}

pub fn run() -> Result<(), WebAppError> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .thread_name("main")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_io()
        .enable_time()
        .build()
        .map_err(WebAppError::TokioRuntimeInit)?;

    let CliArgs { command, format } = CliArgs::parse();
    #[allow(unused_assignments)]
    runtime.block_on(async {
        let _workspace_spec = WorkspaceSpec::WorkingDir;
        let _profile = profile!("default");
        let _flow_id = flow_id!("file");
        let mut cli_output = CliOutput::default();
        if let Some(format) = format {
            cli_output = cli_output.output_format(format);
        }
        #[cfg(feature = "output_colorized")]
        {
            cli_output = cli_output.colorized();
        }

        match command {
            WebAppCommand::Init { slug, version } => {
                AppInitCmd::run(&mut cli_output, slug, version).await?
            }
            WebAppCommand::Profile { command } => match command {
                ProfileCommand::Init { name: _, r#type: _ } => todo!(),
                ProfileCommand::List => todo!(),
                ProfileCommand::Show => todo!(),
            },
            WebAppCommand::Switch { profile: _ } => todo!(),
            WebAppCommand::Fetch => todo!(),
            WebAppCommand::Status => todo!(),
            WebAppCommand::Desired => todo!(),
            WebAppCommand::Diff => todo!(),
            WebAppCommand::Push => todo!(),
            WebAppCommand::Pull => todo!(),
            WebAppCommand::Clean => todo!(),
        }

        Ok::<_, WebAppError>(())
    })
}
