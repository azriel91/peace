use futures::FutureExt;
use peace::{
    cmd::scopes::SingleProfileSingleFlowView,
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    rt::cmds::{sub::StatesSavedReadCmd, EnsureCmd},
    rt_model::{outcomes::CmdOutcome, output::OutputWrite},
};

use crate::{cmds::EnvCmd, model::AppCycleError};

/// Deploys or updates the environment.
#[derive(Debug)]
pub struct EnvDeployCmd;

impl EnvDeployCmd {
    /// Deploys or updates the environment.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    /// * `slug`: Username and repository of the application to download.
    /// * `version`: Version of the application to download.
    /// * `url`: URL to override where to download the application from.
    pub async fn run<O>(output: &mut O) -> Result<(), AppCycleError>
    where
        O: OutputWrite<AppCycleError> + Send,
    {
        let states_saved = EnvCmd::run(output, true, |ctx| {
            StatesSavedReadCmd::exec(ctx).boxed_local()
        })
        .await?;
        EnvCmd::run(output, false, |ctx| {
            async move {
                let states_saved_ref = &states_saved;
                let states_ensured_outcome = EnsureCmd::exec(ctx, states_saved_ref).await?;
                let CmdOutcome {
                    value: states_ensured,
                    errors,
                } = &states_ensured_outcome;

                let states_ensured_raw_map = &***states_ensured;

                let SingleProfileSingleFlowView { output, flow, .. } = ctx.view();
                let states_ensured_presentables = {
                    let states_ensured_presentables = flow
                        .graph()
                        .iter_insertion()
                        .map(|item_spec| {
                            let item_spec_id = item_spec.id();
                            match states_ensured_raw_map.get(item_spec_id) {
                                Some(state_ensured) => (item_spec_id, format!(": {state_ensured}")),
                                None => (item_spec_id, String::from(": <unknown>")),
                            }
                        })
                        .collect::<Vec<_>>();

                    ListNumbered::new(states_ensured_presentables)
                };

                output
                    .present(&(
                        Heading::new(HeadingLevel::Level1, "States Ensured"),
                        states_ensured_presentables,
                        "\n",
                    ))
                    .await?;

                if states_ensured_outcome.is_err() {
                    #[cfg(feature = "error_reporting")]
                    {
                        use peace::miette::{Diagnostic, GraphicalReportHandler};

                        let report_handler = GraphicalReportHandler::new().without_cause_chain();
                        let mut err_buffer = String::new();
                        for (item_spec_id, error) in errors.iter() {
                            #[rustfmt::skip]
                            let diagnostic: &dyn Diagnostic = match error {
                                // AppCycleError::AppCycleUrlBuild { url_candidate, error } => todo!(),
                                // AppCycleError::EnvTypeParseError(_) => todo!(),
                                // AppCycleError::ProfileSwitchToNonExistent { profile_to_switch_to, app_name } => todo!(),
                                // AppCycleError::ProfileToCreateExists { profile_to_create, app_name } => todo!(),

                                AppCycleError::PeaceItemSpecFileDownload(e) => e,
                                AppCycleError::PeaceItemSpecTarX(e) => e,
                                AppCycleError::InstanceProfileItemSpec(e) => e,
                                AppCycleError::IamPolicyItemSpec(e) => e,
                                AppCycleError::IamRoleItemSpec(e) => e,
                                AppCycleError::S3BucketItemSpec(e) => e,
                                AppCycleError::S3ObjectItemSpec(e) => e,
                                AppCycleError::PeaceRtError(e) => e,
                                // AppCycleError::WouldCycleError(_) => todo!(),
                                // AppCycleError::TokioRuntimeInit(_) => todo!(),

                                _ => error,
                            };

                            // Ignore failures when writing errors
                            let (Ok(()) | Err(_)) = output.present(item_spec_id).await;
                            let (Ok(()) | Err(_)) = output.present(":\n").await;
                            let (Ok(()) | Err(_)) =
                                report_handler.render_report(&mut err_buffer, diagnostic);
                            let (Ok(()) | Err(_)) = output.present(&err_buffer).await;
                            let (Ok(()) | Err(_)) = output.present("\n").await;

                            err_buffer.clear();
                        }
                    }

                    #[cfg(not(feature = "error_reporting"))]
                    {
                        use std::error::Error;

                        errors.iter().for_each(|(item_spec_id, error)| {
                            eprintln!("\n{item_spec_id}: {error}");
                            let mut error = error.source();
                            while let Some(source) = error {
                                eprintln!("  caused by: {source}");
                                error = source.source();
                            }
                        });
                    }
                }

                Ok(())
            }
            .boxed_local()
        })
        .await?;

        Ok(())
    }
}
