use std::fmt;

/// Failed to parse CLI colorize from string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliColorizeParseError(pub String);

impl fmt::Display for CliColorizeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to parse CLI colorize from string: `\"{}\"`.\n\
            Valid values are [\"auto\", \"always\", \"never\"]",
            self.0
        )
    }
}

impl std::error::Error for CliColorizeParseError {}
