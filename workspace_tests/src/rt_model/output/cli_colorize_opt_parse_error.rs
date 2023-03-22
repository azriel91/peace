use peace::rt_model::output::CliColorizeOptParseError;

#[test]
fn display_includes_auto_always_never() {
    let error = CliColorizeOptParseError("rara".to_string());

    assert_eq!(
        "Failed to parse CLI colorize from string: `\"rara\"`.\n\
        Valid values are [\"auto\", \"always\", \"never\"]",
        format!("{error}")
    );
}

#[test]
fn clone() {
    let error = CliColorizeOptParseError("rara".to_string());
    #[allow(clippy::redundant_clone)] // https://github.com/rust-lang/rust-clippy/issues/9011
    let error_clone = error.clone();

    assert_eq!(error, error_clone);
}

#[test]
fn debug() {
    let error = CliColorizeOptParseError("rara".to_string());

    assert_eq!(r#"CliColorizeOptParseError("rara")"#, format!("{error:?}"));
}
