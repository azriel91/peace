use std::str::FromStr;

use peace::rt_model::{CliProgressFormat, CliProgressFormatParseError};

#[test]
fn from_str_returns_ok_for_auto() {
    assert_eq!(
        Ok(CliProgressFormat::Auto),
        CliProgressFormat::from_str("auto")
    )
}

#[test]
fn from_str_returns_ok_for_output() {
    assert_eq!(
        Ok(CliProgressFormat::Output),
        CliProgressFormat::from_str("output")
    )
}

#[test]
fn from_str_returns_ok_for_pb() {
    assert_eq!(
        Ok(CliProgressFormat::ProgressBar),
        CliProgressFormat::from_str("pb")
    )
}

#[test]
fn from_str_returns_ok_for_progress_bar() {
    assert_eq!(
        Ok(CliProgressFormat::ProgressBar),
        CliProgressFormat::from_str("progress_bar")
    )
}

#[test]
fn from_str_returns_err_for_unknown_string() {
    assert_eq!(
        Err(CliProgressFormatParseError("rara".to_string())),
        CliProgressFormat::from_str("rara")
    )
}

#[test]
fn clone() {
    let cli_progress_format = CliProgressFormat::Auto;
    let cli_progress_format_clone = cli_progress_format;

    assert_eq!(cli_progress_format, cli_progress_format_clone);
}

#[test]
fn debug() {
    let cli_progress_format = CliProgressFormat::Auto;

    assert_eq!(r#"Auto"#, format!("{cli_progress_format:?}"));
}
