use clap::Parser;
use envman::{
    cmds::{
        EnvCleanCmd, EnvDeployCmd, EnvDesiredCmd, EnvDiffCmd, EnvDiscoverCmd, EnvStatusCmd,
        ProfileInitCmd, ProfileListCmd, ProfileShowCmd, ProfileSwitchCmd,
    },
    model::{
        cli_args::{CliArgs, EnvManCommand, ProfileCommand},
        EnvDiffSelection, EnvManError, ProfileSwitch,
    },
};
use peace::rt_model::output::CliOutput;
use tokio::io::Stdout;

#[cfg(not(feature = "error_reporting"))]
pub fn main() -> Result<(), EnvManError> {
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

    run().map_err(|envman_error| match envman_error {
        EnvManError::PeaceItemFileDownload(err) => peace::miette::Report::from(err),
        EnvManError::PeaceRtError(err) => peace::miette::Report::from(err),
        other => peace::miette::Report::from(other),
    })
}

pub fn run() -> Result<(), EnvManError> {
    let CliArgs {
        command,
        fast,
        format,
        #[cfg(feature = "output_colorized")]
        color,
    } = CliArgs::parse();

    let runtime = if fast {
        tokio::runtime::Builder::new_multi_thread()
    } else {
        tokio::runtime::Builder::new_current_thread()
    }
    .thread_name("main")
    .thread_stack_size(3 * 1024 * 1024)
    .enable_io()
    .enable_time()
    .build()
    .map_err(EnvManError::TokioRuntimeInit)?;

    runtime.block_on(async {
        let mut cli_output = {
            let mut builder = CliOutput::builder();
            if let Some(format) = format {
                builder = builder.with_outcome_format(format);
            }
            #[cfg(feature = "output_colorized")]
            {
                builder = builder.with_colorize(color);
            }

            builder.build()
        };

        match run_command(&mut cli_output, command).await {
            Ok(()) => Ok(()),
            Err(error) => envman::output::errors_present(&mut cli_output, &[error]).await,
        }
    })
}

async fn run_command(
    cli_output: &mut CliOutput<Stdout>,
    command: EnvManCommand,
) -> Result<(), EnvManError> {
    match command {
        EnvManCommand::Init {
            profile,
            flow,
            r#type,
            slug,
            version,
            url,
        } => {
            ProfileInitCmd::run(
                cli_output, profile, flow, r#type, &slug, &version, url, true,
            )
            .await?;
        }
        EnvManCommand::Profile { command } => {
            let command = command.unwrap_or(ProfileCommand::Show);
            match command {
                ProfileCommand::List => ProfileListCmd::run(cli_output).await?,
                ProfileCommand::Show => ProfileShowCmd::run(cli_output).await?,
            }
        }
        EnvManCommand::Switch {
            profile,
            create,
            r#type,
            slug,
            version,
            url,
        } => {
            let profile_switch = if create {
                let Some(((env_type, slug), version)) = r#type.zip(slug).zip(version) else {
                    unreachable!("`clap` should ensure `env_type`, `slug`, and `version` are \
                        `Some` when `create` is `true`.");
                };
                ProfileSwitch::CreateNew {
                    profile,
                    env_type,
                    slug,
                    version,
                    url,
                }
            } else {
                ProfileSwitch::ToExisting { profile }
            };
            ProfileSwitchCmd::run(cli_output, profile_switch).await?
        }
        EnvManCommand::Discover => EnvDiscoverCmd::run(cli_output).await?,
        EnvManCommand::Status => EnvStatusCmd::run(cli_output).await?,
        EnvManCommand::Desired => EnvDesiredCmd::run(cli_output).await?,
        EnvManCommand::Diff {
            profile_a,
            profile_b,
        } => {
            let env_diff_selection = profile_a
                .zip(profile_b)
                .map(
                    |(profile_a, profile_b)| EnvDiffSelection::DiffProfilesCurrent {
                        profile_a,
                        profile_b,
                    },
                )
                .unwrap_or(EnvDiffSelection::CurrentAndDesired);

            EnvDiffCmd::run(cli_output, env_diff_selection).await?
        }
        EnvManCommand::Deploy => EnvDeployCmd::run(cli_output).await?,
        EnvManCommand::Clean => EnvCleanCmd::run(cli_output).await?,
    }

    Ok::<_, EnvManError>(())
}
