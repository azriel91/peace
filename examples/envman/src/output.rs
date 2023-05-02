use peace::{
    cfg::ItemSpecId,
    fmt::presentable::{Heading, HeadingLevel},
    rt_model::{output::OutputWrite, IndexMap},
};

use crate::model::EnvManError;

/// Presents errors.
pub async fn errors_present<O>(output: &mut O, errors: &[EnvManError]) -> Result<(), EnvManError>
where
    O: OutputWrite<EnvManError>,
{
    output
        .present(&Heading::new(HeadingLevel::Level1, "Errors"))
        .await?;

    #[cfg(feature = "error_reporting")]
    {
        use peace::miette::{Diagnostic, GraphicalReportHandler};

        let report_handler = GraphicalReportHandler::new().without_cause_chain();
        let mut err_buffer = String::new();
        for error in errors.iter() {
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
            let (Ok(()) | Err(_)) = report_handler.render_report(&mut err_buffer, diagnostic);
            let (Ok(()) | Err(_)) = output.present(&err_buffer).await;
            let (Ok(()) | Err(_)) = output.present("\n").await;

            err_buffer.clear();
        }
    }

    #[cfg(not(feature = "error_reporting"))]
    {
        use std::error::Error;

        errors.iter().for_each(|error| {
            eprintln!("\n{error}");
            let mut error = error.source();
            while let Some(source) = error {
                eprintln!("  caused by: {source}");
                error = source.source();
            }
        });
    }

    Ok(())
}

/// Presents item spec errors.
pub async fn item_spec_errors_present<O>(
    output: &mut O,
    errors: &IndexMap<ItemSpecId, EnvManError>,
) -> Result<(), EnvManError>
where
    O: OutputWrite<EnvManError>,
{
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
            let (Ok(()) | Err(_)) = report_handler.render_report(&mut err_buffer, diagnostic);
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

    Ok(())
}
