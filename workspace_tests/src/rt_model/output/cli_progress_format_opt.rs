use std::str::FromStr;

use peace::rt_model::output::{CliProgressFormatOpt, CliProgressFormatOptParseError};

#[test]
fn from_str_returns_ok_for_auto() {
    assert_eq!(
        Ok(CliProgressFormatOpt::Auto),
        CliProgressFormatOpt::from_str("auto")
    )
}

#[test]
fn from_str_returns_ok_for_outcome() {
    assert_eq!(
        Ok(CliProgressFormatOpt::Outcome),
        CliProgressFormatOpt::from_str("outcome")
    )
}

#[test]
fn from_str_returns_ok_for_pb() {
    assert_eq!(
        Ok(CliProgressFormatOpt::ProgressBar),
        CliProgressFormatOpt::from_str("pb")
    )
}

#[test]
fn from_str_returns_ok_for_progress_bar() {
    assert_eq!(
        Ok(CliProgressFormatOpt::ProgressBar),
        CliProgressFormatOpt::from_str("progress_bar")
    )
}

#[test]
fn from_str_returns_err_for_unknown_string() {
    assert_eq!(
        Err(CliProgressFormatOptParseError("rara".to_string())),
        CliProgressFormatOpt::from_str("rara")
    )
}

#[test]
fn clone() {
    let cli_progress_format = CliProgressFormatOpt::Auto;
    let cli_progress_format_clone = cli_progress_format;

    assert_eq!(cli_progress_format, cli_progress_format_clone);
}

#[test]
fn debug() {
    let cli_progress_format = CliProgressFormatOpt::Auto;

    assert_eq!(r#"Auto"#, format!("{cli_progress_format:?}"));
}
