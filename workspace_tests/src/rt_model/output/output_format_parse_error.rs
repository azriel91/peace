use peace::rt_model::output::OutputFormatParseError;

#[test]
fn display_includes_text_yaml_json() {
    let error = OutputFormatParseError("rara".to_string());

    assert_eq!(
        r#"Failed to parse output format from string: `"rara"`. Valid values are ["text", "yaml", "json"]"#,
        format!("{error}")
    );
}

#[test]
fn clone() {
    let error = OutputFormatParseError("rara".to_string());
    #[allow(clippy::redundant_clone)] // https://github.com/rust-lang/rust-clippy/issues/9011
    let error_clone = error.clone();

    assert_eq!(error, error_clone);
}

#[test]
fn debug() {
    let error = OutputFormatParseError("rara".to_string());

    assert_eq!(r#"OutputFormatParseError("rara")"#, format!("{error:?}"));
}
