use std::net::SocketAddr;

use clap::Parser;
use envman::{
    cmds::{
        EnvCleanCmd, EnvDeployCmd, EnvDiffCmd, EnvDiscoverCmd, EnvGoalCmd, EnvStatusCmd,
        ProfileInitCmd, ProfileListCmd, ProfileShowCmd, ProfileSwitchCmd,
    },
    model::{
        cli_args::{CliArgs, EnvManCommand, ProfileCommand},
        EnvDiffSelection, EnvManError, ProfileSwitch,
    },
};
use peace::cli::output::CliOutput;
use tokio::io::Stdout;

pub fn run() -> Result<(), EnvManError> {
    let CliArgs {
        command,
        fast,
        format,
        color,
        debug,
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
            let mut builder = CliOutput::builder().with_colorize(color);
            if let Some(format) = format {
                builder = builder.with_outcome_format(format);
            }

            builder.build()
        };

        match run_command(&mut cli_output, command, debug).await {
            Ok(()) => Ok(()),
            Err(error) => envman::output::errors_present(&mut cli_output, &[error]).await,
        }
    })
}

async fn run_command(
    cli_output: &mut CliOutput<Stdout>,
    command: EnvManCommand,
    debug: bool,
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
                    unreachable!(
                        "`clap` should ensure `env_type`, `slug`, and `version` are \
                        `Some` when `create` is `true`."
                    );
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
        EnvManCommand::Discover => EnvDiscoverCmd::run(cli_output, debug).await?,
        EnvManCommand::Status => EnvStatusCmd::run(cli_output).await?,
        EnvManCommand::Goal => EnvGoalCmd::run(cli_output).await?,
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
                .unwrap_or(EnvDiffSelection::CurrentAndGoal);

            EnvDiffCmd::run(cli_output, env_diff_selection).await?
        }
        EnvManCommand::Deploy => EnvDeployCmd::run(cli_output, debug).await?,
        EnvManCommand::Clean => EnvCleanCmd::run(cli_output, debug).await?,
        #[cfg(feature = "web_server")]
        EnvManCommand::Web { address, port } => {
            use peace::webi::output::WebiOutput;

            let webi_output = WebiOutput::new(Some(SocketAddr::from((address, port))));
            webi_output.start().await?;

            // WebServer::start(Some(SocketAddr::from((address, port)))).await?
        }
    }

    Ok::<_, EnvManError>(())
}
