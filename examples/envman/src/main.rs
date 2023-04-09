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
use peace::{
    cfg::{flow_id, profile, FlowId, Profile},
    rt_model::{output::CliOutput, WorkspaceSpec},
};

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
        EnvManError::PeaceItemSpecFileDownload(err) => peace::miette::Report::from(err),
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

    #[allow(unused_assignments)]
    runtime.block_on(async {
        let _workspace_spec = WorkspaceSpec::WorkingDir;
        let _profile = profile!("default");
        let _flow_id = flow_id!("file");
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

        match command {
            EnvManCommand::Init {
                profile,
                r#type,
                slug,
                version,
                url,
            } => {
                ProfileInitCmd::run(&mut cli_output, profile, r#type, &slug, &version, url).await?;
            }
            EnvManCommand::Profile { command } => {
                let command = command.unwrap_or(ProfileCommand::Show);
                match command {
                    ProfileCommand::List => ProfileListCmd::run(&mut cli_output).await?,
                    ProfileCommand::Show => ProfileShowCmd::run(&mut cli_output).await?,
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
                ProfileSwitchCmd::run(&mut cli_output, profile_switch).await?
            }
            EnvManCommand::Discover => EnvDiscoverCmd::run(&mut cli_output).await?,
            EnvManCommand::Status => EnvStatusCmd::run(&mut cli_output).await?,
            EnvManCommand::Desired => EnvDesiredCmd::run(&mut cli_output).await?,
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

                EnvDiffCmd::run(&mut cli_output, env_diff_selection).await?
            }
            EnvManCommand::Deploy => EnvDeployCmd::run(&mut cli_output).await?,
            EnvManCommand::Clean => EnvCleanCmd::run(&mut cli_output).await?,
        }

        Ok::<_, EnvManError>(())
    })
}
