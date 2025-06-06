use std::net::SocketAddr;

use clap::Parser;
use envman::{
    cmds::{
        CmdOpts, EnvCleanCmd, EnvDeployCmd, EnvDiffCmd, EnvDiscoverCmd, EnvGoalCmd, EnvStatusCmd,
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

                #[cfg(feature = "output_progress")]
                {
                    use peace::cli::output::CliProgressFormatOpt;
                    builder = builder.with_progress_format(CliProgressFormatOpt::Outcome);
                }
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
            use futures::FutureExt;
            use peace::{
                cmd_ctx::CmdCtxSpsfFields,
                cmd_model::CmdOutcome,
                webi::output::{CmdExecSpawnCtx, FlowWebiFns, WebiServer},
                webi_components::ChildrenFn,
            };

            use envman::{
                cmds::EnvCmd,
                flows::EnvDeployFlow,
                web_components::{CmdExecRequest, EnvDeployHome},
            };

            let flow = EnvDeployFlow::flow()
                .await
                .expect("Failed to instantiate EnvDeployFlow.");

            let flow_webi_fns = FlowWebiFns {
                flow: flow.clone(),
                outcome_info_graph_fn: Box::new(|webi_output, outcome_info_graph_gen| {
                    async move {
                        let cmd_ctx = EnvCmd::cmd_ctx(webi_output)
                            .await
                            .expect("Expected CmdCtx to be successfully constructed.");

                        // TODO: consolidate the `flow` above with this?
                        let CmdCtxSpsfFields {
                            flow,
                            params_specs,
                            resources,
                            ..
                        } = cmd_ctx.fields();

                        outcome_info_graph_gen(flow, params_specs, resources)
                    }
                    .boxed_local()
                }),
                cmd_exec_spawn_fn: Box::new(|mut webi_output, cmd_exec_request| {
                    use peace::rt::cmds::{
                        ApplyStoredStateSync, CleanCmd, EnsureCmd, StatesDiscoverCmd,
                    };
                    let cmd_exec_task = async move {
                        let mut cli_output = CliOutput::builder().build();
                        let cli_output = &mut cli_output;

                        let (cmd_error, item_errors) = match cmd_exec_request {
                            CmdExecRequest::Discover => {
                                eprintln!("Running discover.");
                                let result =
                                    EnvCmd::run(&mut webi_output, CmdOpts::default(), |cmd_ctx| {
                                        async { StatesDiscoverCmd::current_and_goal(cmd_ctx).await }
                                            .boxed_local()
                                    })
                                    .await;

                                match result {
                                    Ok(cmd_outcome) => {
                                        if let CmdOutcome::ItemError { errors, .. } = cmd_outcome {
                                            (None, Some(errors))
                                        } else {
                                            (None, None)
                                        }
                                    }
                                    Err(error) => (Some(error), None),
                                }
                            }
                            CmdExecRequest::Ensure => {
                                eprintln!("Running ensure.");
                                let result =
                                    EnvCmd::run(&mut webi_output, CmdOpts::default(), |cmd_ctx| {
                                        async {
                                            EnsureCmd::exec_with(
                                                cmd_ctx,
                                                ApplyStoredStateSync::Current,
                                            )
                                            .await
                                        }
                                        .boxed_local()
                                    })
                                    .await;

                                match result {
                                    Ok(cmd_outcome) => {
                                        if let CmdOutcome::ItemError { errors, .. } = cmd_outcome {
                                            (None, Some(errors))
                                        } else {
                                            (None, None)
                                        }
                                    }
                                    Err(error) => (Some(error), None),
                                }
                            }
                            CmdExecRequest::Clean => {
                                eprintln!("Running clean.");
                                let result =
                                    EnvCmd::run(&mut webi_output, CmdOpts::default(), |cmd_ctx| {
                                        async {
                                            CleanCmd::exec_with(
                                                cmd_ctx,
                                                ApplyStoredStateSync::Current,
                                            )
                                            .await
                                        }
                                        .boxed_local()
                                    })
                                    .await;

                                match result {
                                    Ok(cmd_outcome) => {
                                        if let CmdOutcome::ItemError { errors, .. } = cmd_outcome {
                                            (None, Some(errors))
                                        } else {
                                            (None, None)
                                        }
                                    }
                                    Err(error) => (Some(error), None),
                                }
                            }
                        };

                        if let Some(cmd_error) = cmd_error {
                            let _ = envman::output::errors_present(cli_output, &[cmd_error]).await;
                        }
                        if let Some(item_errors) = item_errors.as_ref() {
                            let _ =
                                envman::output::item_errors_present(cli_output, item_errors).await;
                        }
                    }
                    .boxed_local();

                    CmdExecSpawnCtx {
                        interrupt_tx: None,
                        cmd_exec_task,
                    }
                }),
            };

            WebiServer::start(
                env!("CARGO_CRATE_NAME").to_string(),
                Some(SocketAddr::from((address, port))),
                ChildrenFn::new(EnvDeployHome),
                flow_webi_fns,
            )
            .await?;
        }
    }

    Ok::<_, EnvManError>(())
}
