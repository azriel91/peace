use peace::{
    cfg::ItemId,
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
            let mut diagnostic_opt: Option<&dyn Diagnostic> = Some(error);
            while let Some(diagnostic) = diagnostic_opt {
                if diagnostic.help().is_some()
                    || diagnostic.labels().is_some()
                    || diagnostic.diagnostic_source().is_none()
                {
                    // Ignore failures when writing errors
                    let (Ok(()) | Err(_)) =
                        report_handler.render_report(&mut err_buffer, diagnostic);
                    let (Ok(()) | Err(_)) = output.present(&err_buffer).await;
                    let (Ok(()) | Err(_)) = output.present("\n").await;
                }

                diagnostic_opt = diagnostic.diagnostic_source();
            }

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

/// Presents item errors.
pub async fn item_errors_present<O>(
    output: &mut O,
    errors: &IndexMap<ItemId, EnvManError>,
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
        for (item_id, error) in errors.iter() {
            // Ignore failures when writing errors
            let (Ok(()) | Err(_)) = output.present(item_id).await;
            let (Ok(()) | Err(_)) = output.present(":\n").await;

            let mut diagnostic_opt: Option<&dyn Diagnostic> = Some(error);
            while let Some(diagnostic) = diagnostic_opt {
                if diagnostic.help().is_some()
                    || diagnostic.labels().is_some()
                    || diagnostic.diagnostic_source().is_none()
                {
                    // Ignore failures when writing errors
                    let (Ok(()) | Err(_)) =
                        report_handler.render_report(&mut err_buffer, diagnostic);
                    let (Ok(()) | Err(_)) = output.present(&err_buffer).await;
                    let (Ok(()) | Err(_)) = output.present("\n").await;
                }

                diagnostic_opt = diagnostic.diagnostic_source();
            }

            err_buffer.clear();
        }
    }

    #[cfg(not(feature = "error_reporting"))]
    {
        use std::error::Error;

        errors.iter().for_each(|(item_id, error)| {
            eprintln!("\n{item_id}: {error}");
            let mut error = error.source();
            while let Some(source) = error {
                eprintln!("  caused by: {source}");
                error = source.source();
            }
        });
    }

    Ok(())
}
