use peace::{
    cli::output::{CliColorize, CliColorizeOpt, CliOutputBuilder},
    cli_model::OutputFormat,
};

#[cfg(feature = "output_progress")]
use peace::cli::output::{CliOutputTarget, CliProgressFormat, CliProgressFormatOpt};

#[tokio::test]
async fn new_uses_sensible_defaults() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new();

    assert_eq!(OutputFormat::Text, builder.outcome_format());
    assert_eq!(CliColorizeOpt::Auto, builder.colorize());
    #[cfg(feature = "output_progress")]
    assert_eq!(&CliOutputTarget::Stderr, builder.progress_target());
    #[cfg(feature = "output_progress")]
    assert_eq!(CliProgressFormatOpt::Auto, builder.progress_format());
    Ok(())
}

#[tokio::test]
async fn with_outcome_format_sets_outcome_format() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new().with_outcome_format(OutputFormat::Yaml);

    assert_eq!(OutputFormat::Yaml, builder.outcome_format());
    Ok(())
}

#[tokio::test]
async fn with_colorize_sets_colorize() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new().with_colorize(CliColorizeOpt::Always);

    assert_eq!(CliColorizeOpt::Always, builder.colorize());
    Ok(())
}

#[cfg(feature = "output_progress")]
#[tokio::test]
async fn with_progress_target_sets_progress_target() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new().with_progress_target(CliOutputTarget::Stdout);

    assert_eq!(&CliOutputTarget::Stdout, builder.progress_target());
    Ok(())
}

#[cfg(feature = "output_progress")]
#[tokio::test]
async fn with_progress_format_sets_progress_format() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new().with_progress_format(CliProgressFormatOpt::Outcome);

    assert_eq!(CliProgressFormatOpt::Outcome, builder.progress_format());
    Ok(())
}

#[tokio::test]
async fn build_passes_through_outcome_format() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new().with_outcome_format(OutputFormat::Yaml);

    let cli_output = builder.build();

    assert_eq!(OutputFormat::Yaml, cli_output.outcome_format());
    Ok(())
}

#[tokio::test]
async fn build_passes_through_colorize() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new().with_colorize(CliColorizeOpt::Always);

    let cli_output = builder.build();

    assert_eq!(CliColorize::Colored, cli_output.colorize());
    Ok(())
}

#[cfg(feature = "output_progress")]
#[tokio::test]
async fn build_passes_through_progress_target() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new().with_progress_target(CliOutputTarget::Stdout);

    let cli_output = builder.build();

    assert_eq!(&CliOutputTarget::Stdout, cli_output.progress_target());
    Ok(())
}

#[cfg(feature = "output_progress")]
#[tokio::test]
async fn build_passes_through_progress_format() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new().with_progress_format(CliProgressFormatOpt::Outcome);

    let cli_output = builder.build();

    assert_eq!(CliProgressFormat::Outcome, cli_output.progress_format());
    Ok(())
}

// Auto default tests

// TODO: Test interactive terminal.
//
// Options:
//
// * Implement a trait for `tokio::io::Stdout`, `tokio::io::Stderr`, `Vec<u8>`
// * Build a small executable and run it via `process::Command`.

#[tokio::test]
async fn build_colorize_auto_passes_uncolored_for_non_interactive_terminal(
) -> Result<(), Box<dyn std::error::Error>> {
    let cli_output = CliOutputBuilder::new().build();

    assert_eq!(CliColorize::Uncolored, cli_output.colorize());
    Ok(())
}

#[cfg(feature = "output_progress")]
#[tokio::test]
async fn build_progress_format_auto_passes_stderr_for_non_interactive_terminal(
) -> Result<(), Box<dyn std::error::Error>> {
    let cli_output = CliOutputBuilder::new().build();

    assert_eq!(CliProgressFormat::Outcome, cli_output.progress_format());
    Ok(())
}
