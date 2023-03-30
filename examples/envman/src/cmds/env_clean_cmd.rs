use futures::FutureExt;
use peace::{
    cmd::scopes::SingleProfileSingleFlowView,
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    rt::cmds::{sub::StatesSavedReadCmd, CleanCmd},
    rt_model::{outcomes::CmdOutcome, output::OutputWrite},
};

use crate::{cmds::EnvCmd, model::EnvManError};

/// Cleans up (deletes) the environment.
#[derive(Debug)]
pub struct EnvCleanCmd;

impl EnvCleanCmd {
    /// Cleans up (deletes) the environment.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    /// * `slug`: Username and repository of the application to download.
    /// * `version`: Version of the application to download.
    /// * `url`: URL to override where to download the application from.
    pub async fn run<O>(output: &mut O) -> Result<(), EnvManError>
    where
        O: OutputWrite<EnvManError> + Send,
    {
        let states_saved = EnvCmd::run(output, true, |ctx| {
            StatesSavedReadCmd::exec(ctx).boxed_local()
        })
        .await?;
        EnvCmd::run(output, false, |ctx| {
            async move {
                let states_saved_ref = &states_saved;
                let states_cleaned_outcome = CleanCmd::exec(ctx, states_saved_ref).await?;
                let CmdOutcome {
                    value: states_cleaned,
                    errors,
                } = &states_cleaned_outcome;
                let SingleProfileSingleFlowView { output, flow, .. } = ctx.view();

                if states_cleaned_outcome.is_ok() {
                    let states_cleaned_raw_map = &***states_cleaned;

                    let states_cleaned_presentables = {
                        let states_cleaned_presentables = flow
                            .graph()
                            .iter_insertion()
                            .map(|item_spec| {
                                let item_spec_id = item_spec.id();
                                match states_cleaned_raw_map.get(item_spec_id) {
                                    Some(state_cleaned) => {
                                        (item_spec_id, format!(": {state_cleaned}"))
                                    }
                                    None => (item_spec_id, String::from(": <unknown>")),
                                }
                            })
                            .collect::<Vec<_>>();

                        ListNumbered::new(states_cleaned_presentables)
                    };

                    output
                        .present(&(
                            Heading::new(HeadingLevel::Level1, "States Cleaned"),
                            states_cleaned_presentables,
                            "\n",
                        ))
                        .await?;
                } else {
                    output
                        .present(&Heading::new(HeadingLevel::Level1, "Errors"))
                        .await?;

                    #[cfg(feature = "error_reporting")]
                    {
                        use peace::miette::{Diagnostic, GraphicalReportHandler};

                        let report_handler = GraphicalReportHandler::new().without_cause_chain();
                        let mut err_buffer = String::new();
                        for (item_spec_id, error) in errors.iter() {
                            #[rustfmt::skip]
                            let diagnostic: &dyn Diagnostic = match error {
                                // EnvManError::EnvManUrlBuild { url_candidate, error } => todo!(),
                                // EnvManError::EnvTypeParseError(_) => todo!(),
                                // EnvManError::ProfileSwitchToNonExistent { profile_to_switch_to, app_name } => todo!(),
                                // EnvManError::ProfileToCreateExists { profile_to_create, app_name } => todo!(),

                                EnvManError::PeaceItemSpecFileDownload(e) => e,
                                EnvManError::PeaceItemSpecTarX(e) => e,
                                EnvManError::InstanceProfileItemSpec(e) => e,
                                EnvManError::IamPolicyItemSpec(e) => e,
                                EnvManError::IamRoleItemSpec(e) => e,
                                EnvManError::S3BucketItemSpec(e) => e,
                                EnvManError::S3ObjectItemSpec(e) => e,
                                EnvManError::PeaceRtError(e) => e,
                                // EnvManError::WouldCycleError(_) => todo!(),
                                // EnvManError::TokioRuntimeInit(_) => todo!(),

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
