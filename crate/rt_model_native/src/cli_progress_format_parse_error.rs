use std::fmt;

/// Failed to parse CLI progress format from string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliProgressFormatParseError(pub String);

impl fmt::Display for CliProgressFormatParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to parse CLI progress format from string: `\"{}\"`.\n\
            Valid values are [\"auto\", \"output\", \"pb\", \"progress_bar\"]",
            self.0
        )
    }
}

impl std::error::Error for CliProgressFormatParseError {}