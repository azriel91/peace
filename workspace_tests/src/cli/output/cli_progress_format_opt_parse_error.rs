use peace::cli::output::CliProgressFormatOptParseError;

#[test]
fn display_includes_auto_output_pb_progress_bar() {
    let error = CliProgressFormatOptParseError("rara".to_string());

    assert_eq!(
        "Failed to parse CLI progress format from string: `\"rara\"`.\n\
        Valid values are [\"auto\", \"outcome\", \"pb\", \"progress_bar\", \"none\"]",
        format!("{error}")
    );
}

#[test]
fn clone() {
    let error = CliProgressFormatOptParseError("rara".to_string());
    #[allow(clippy::redundant_clone)] // https://github.com/rust-lang/rust-clippy/issues/9011
    let error_clone = error.clone();

    assert_eq!(error, error_clone);
}

#[test]
fn debug() {
    let error = CliProgressFormatOptParseError("rara".to_string());

    assert_eq!(
        r#"CliProgressFormatOptParseError("rara")"#,
        format!("{error:?}")
    );
}
