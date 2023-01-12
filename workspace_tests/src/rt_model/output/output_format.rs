use std::str::FromStr;

use peace::rt_model::output::{OutputFormat, OutputFormatParseError};

#[test]
fn from_str_returns_ok_for_text() {
    assert_eq!(Ok(OutputFormat::Text), OutputFormat::from_str("text"))
}

#[test]
fn from_str_returns_ok_for_yaml() {
    assert_eq!(Ok(OutputFormat::Yaml), OutputFormat::from_str("yaml"))
}

#[cfg(not(feature = "output_json"))]
#[test]
fn from_str_returns_err_for_json_when_json_feature_is_not_enabled() {
    assert_eq!(
        Err(OutputFormatParseError("json".to_string())),
        OutputFormat::from_str("json")
    )
}

#[cfg(feature = "output_json")]
#[test]
fn from_str_returns_ok_for_json_when_json_feature_is_enabled() {
    assert_eq!(Ok(OutputFormat::Json), OutputFormat::from_str("json"))
}

#[test]
fn from_str_returns_err_for_unknown_string() {
    assert_eq!(
        Err(OutputFormatParseError("rara".to_string())),
        OutputFormat::from_str("rara")
    )
}

#[test]
fn clone() {
    let output_format = OutputFormat::Text;
    let output_format_clone = output_format;

    assert_eq!(output_format, output_format_clone);
}

#[test]
fn debug() {
    let output_format = OutputFormat::Text;

    assert_eq!(r#"Text"#, format!("{output_format:?}"));
}
