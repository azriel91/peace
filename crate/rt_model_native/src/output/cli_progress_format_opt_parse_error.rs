use std::fmt;

/// Failed to parse CLI progress format from string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliProgressFormatOptParseError(pub String);

impl fmt::Display for CliProgressFormatOptParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to parse CLI progress format from string: `\"{}\"`.\n\
            Valid values are [\"auto\", \"outcome\", \"pb\", \"progress_bar\", \"none\"]",
            self.0
        )
    }
}

impl std::error::Error for CliProgressFormatOptParseError {}
