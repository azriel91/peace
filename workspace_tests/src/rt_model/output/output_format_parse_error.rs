use peace::rt_model::output::OutputFormatParseError;

#[cfg(not(feature = "output_json"))]
#[test]
fn display_includes_text_yaml_when_json_feature_is_not_enabled() {
    let error = OutputFormatParseError("rara".to_string());

    assert_eq!(
        r#"Failed to parse output format from string: `"rara"`. Valid values are ["text", "yaml"]"#,
        format!("{error}")
    );
}

#[cfg(feature = "output_json")]
#[test]
fn display_includes_text_yaml_when_json_feature_is_enabled() {
    let error = OutputFormatParseError("rara".to_string());

    assert_eq!(
        r#"Failed to parse output format from string: `"rara"`. Valid values are ["text", "yaml", "json"]"#,
        format!("{error}")
    );
}

#[test]
fn clone() {
    let error = OutputFormatParseError("rara".to_string());
    let error_clone = error.clone();

    assert_eq!(error, error_clone);
}

#[test]
fn debug() {
    let error = OutputFormatParseError("rara".to_string());

    assert_eq!(r#"OutputFormatParseError("rara")"#, format!("{error:?}"));
}
