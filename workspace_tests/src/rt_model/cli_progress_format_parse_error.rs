use peace::rt_model::CliProgressFormatParseError;

#[test]
fn display_includes_auto_output_pb_progress_bar() {
    let error = CliProgressFormatParseError("rara".to_string());

    assert_eq!(
        "Failed to parse CLI progress format from string: `\"rara\"`.\n\
        Valid values are [\"auto\", \"output\", \"pb\", \"progress_bar\"]",
        format!("{error}")
    );
}

#[test]
fn clone() {
    let error = CliProgressFormatParseError("rara".to_string());
    let error_clone = error.clone();

    assert_eq!(error, error_clone);
}

#[test]
fn debug() {
    let error = CliProgressFormatParseError("rara".to_string());

    assert_eq!(
        r#"CliProgressFormatParseError("rara")"#,
        format!("{error:?}")
    );
}
