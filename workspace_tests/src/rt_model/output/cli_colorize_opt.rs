use std::str::FromStr;

use peace::rt_model::output::{CliColorizeOpt, CliColorizeOptParseError};

#[test]
fn from_str_returns_ok_for_auto() {
    assert_eq!(Ok(CliColorizeOpt::Auto), CliColorizeOpt::from_str("auto"))
}

#[test]
fn from_str_returns_ok_for_always() {
    assert_eq!(
        Ok(CliColorizeOpt::Always),
        CliColorizeOpt::from_str("always")
    )
}

#[test]
fn from_str_returns_ok_for_never() {
    assert_eq!(Ok(CliColorizeOpt::Never), CliColorizeOpt::from_str("never"))
}

#[test]
fn from_str_returns_err_for_unknown_string() {
    assert_eq!(
        Err(CliColorizeOptParseError("rara".to_string())),
        CliColorizeOpt::from_str("rara")
    )
}

#[test]
fn clone() {
    let cli_colorize = CliColorizeOpt::Auto;
    let cli_colorize_clone = cli_colorize;

    assert_eq!(cli_colorize, cli_colorize_clone);
}

#[test]
fn debug() {
    let cli_colorize = CliColorizeOpt::Auto;

    assert_eq!(r#"Auto"#, format!("{cli_colorize:?}"));
}
