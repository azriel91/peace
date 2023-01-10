use peace::rt_model::output::{CliOutputBuilder, OutputFormat};

#[cfg(feature = "output_colorized")]
use peace::rt_model::output::{CliColorize, CliColorizeUsed};
#[cfg(feature = "output_progress")]
use peace::rt_model::output::{CliOutputTarget, CliProgressFormat, CliProgressFormatUsed};

#[tokio::test]
async fn new_uses_sensible_defaults() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new();

    assert_eq!(OutputFormat::Text, builder.outcome_format());
    #[cfg(feature = "output_colorized")]
    assert_eq!(CliColorize::Auto, builder.colorize());
    #[cfg(feature = "output_progress")]
    assert_eq!(CliOutputTarget::Stderr, builder.progress_target());
    #[cfg(feature = "output_progress")]
    assert_eq!(CliProgressFormat::Auto, builder.progress_format());
    Ok(())
}

#[tokio::test]
async fn with_outcome_format_sets_outcome_format() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new().with_outcome_format(OutputFormat::Yaml);

    assert_eq!(OutputFormat::Yaml, builder.outcome_format());
    Ok(())
}

#[cfg(feature = "output_colorized")]
#[tokio::test]
async fn with_colorize_sets_colorize() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new().with_colorize(CliColorize::Always);

    assert_eq!(CliColorize::Always, builder.colorize());
    Ok(())
}

#[cfg(feature = "output_progress")]
#[tokio::test]
async fn with_progress_target_sets_progress_target() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new().with_progress_target(CliOutputTarget::Stdout);

    assert_eq!(CliOutputTarget::Stdout, builder.progress_target());
    Ok(())
}

#[cfg(feature = "output_progress")]
#[tokio::test]
async fn with_progress_format_sets_progress_format() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new().with_progress_format(CliProgressFormat::Output);

    assert_eq!(CliProgressFormat::Output, builder.progress_format());
    Ok(())
}

#[tokio::test]
async fn build_passes_through_outcome_format() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new().with_outcome_format(OutputFormat::Yaml);

    let cli_output = builder.build();

    assert_eq!(OutputFormat::Yaml, cli_output.outcome_format());
    Ok(())
}

#[cfg(feature = "output_colorized")]
#[tokio::test]
async fn build_passes_through_colorize() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new().with_colorize(CliColorize::Always);

    let cli_output = builder.build();

    assert_eq!(CliColorizeUsed::Colored, cli_output.colorize());
    Ok(())
}

#[cfg(feature = "output_progress")]
#[tokio::test]
async fn build_passes_through_progress_target() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new().with_progress_target(CliOutputTarget::Stdout);

    let cli_output = builder.build();

    assert_eq!(CliOutputTarget::Stdout, cli_output.progress_target());
    Ok(())
}

#[cfg(feature = "output_progress")]
#[tokio::test]
async fn build_passes_through_progress_format() -> Result<(), Box<dyn std::error::Error>> {
    let builder = CliOutputBuilder::new().with_progress_format(CliProgressFormat::Output);

    let cli_output = builder.build();

    assert_eq!(CliProgressFormatUsed::Output, cli_output.progress_format());
    Ok(())
}

// Auto default tests

// TODO: Test interactive terminal.
//
// Options:
//
// * Implement a trait for `tokio::io::Stdout`, `tokio::io::Stderr`, `Vec<u8>`
// * Build a small executable and run it via `process::Command`.

#[cfg(feature = "output_colorized")]
#[tokio::test]
async fn build_colorize_auto_passes_uncolored_for_non_interactive_terminal()
-> Result<(), Box<dyn std::error::Error>> {
    let cli_output = CliOutputBuilder::new().build();

    assert_eq!(CliColorizeUsed::Uncolored, cli_output.colorize());
    Ok(())
}

#[cfg(feature = "output_progress")]
#[tokio::test]
async fn build_progress_format_auto_passes_stderr_for_non_interactive_terminal()
-> Result<(), Box<dyn std::error::Error>> {
    let cli_output = CliOutputBuilder::new().build();

    assert_eq!(CliProgressFormatUsed::Output, cli_output.progress_format());
    Ok(())
}
