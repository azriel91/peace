/// Returns the error location and message to pass to miette.
///
/// TODO: Replace hack.
///
/// The `location()` reported in the error is incorrect, due to
/// <https://github.com/dtolnay/serde-yaml/issues/153>
///
/// In certain cases, we can reverse engineer the error from the
/// `Display` string of the error.
pub(crate) fn error_and_context(
    file_contents: &str,
    error: &serde_yaml::Error,
) -> (
    Option<miette::SourceOffset>,
    String,
    Option<miette::SourceOffset>,
) {
    let error_string = format!("{error}");
    let (error_span, context_span) = match error.location().map(|error_location| {
        (
            error_location.index(),
            error_location.line(),
            error_location.column(),
        )
    }) {
        // The `error_location` is not the true location. Extract it from the `Display` string.
        //
        // See:
        //
        // * <https://github.com/dtolnay/serde-yaml/blob/0.9.14/src/libyaml/error.rs#L65-L84>
        // * <https://github.com/dtolnay/serde-yaml/blob/0.9.14/src/libyaml/error.rs#L141>
        //
        // Example error strings (truncated the beginning):
        //
        // ```text
        // missing field `path` at line 2 column 12 at line 2 column 3
        // unknown variant `~`, expected one of `a`, `b` at line 2 column 11 at line 2 column 11 at line 2 column 3
        // ```
        Some((0, 1, 1)) => {
            // TODO: This may also be "at position 123", but we don't support that yet.
            let mut line_column_pairs =
                error_string.rsplit(" at line ").filter_map(|line_column| {
                    let mut line_column_split = line_column.split(" column ");
                    let line = line_column_split
                        .next()
                        .map(str::parse::<usize>)
                        .and_then(Result::ok);
                    let column = line_column_split
                        .next()
                        .map(str::parse::<usize>)
                        .and_then(Result::ok);

                    if let (Some(line), Some(column)) = (line, column) {
                        Some((line, column))
                    } else {
                        None
                    }
                });

            let last_mark = line_column_pairs.next().map(|(line, column)| {
                miette::SourceOffset::from_location(file_contents, line, column)
            });
            let second_to_last_mark = line_column_pairs.next().map(|(line, column)| {
                miette::SourceOffset::from_location(file_contents, line, column)
            });

            match (second_to_last_mark, last_mark) {
                (error_span @ Some(_), context_span @ Some(_)) => (error_span, context_span),
                (None, error_span @ Some(_)) => (error_span, None),
                (Some(_), None) | (None, None) => (None, None),
            }
        }
        Some((_, line, column)) => (
            Some(miette::SourceOffset::from_location(
                file_contents,
                line,
                column,
            )),
            None,
        ),
        None => (None, None),
    };

    let error_message = error_string
        .split(" at ")
        .next()
        .map(str::to_string)
        .unwrap_or(error_string);
    (error_span, error_message, context_span)
}
