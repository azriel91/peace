use std::fmt;

/// Failed to parse output format from string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputFormatParseError(pub String);

impl fmt::Display for OutputFormatParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"Failed to parse output format from string: `"{}"`. Valid values are ["text", "yaml", "json"]"#,
            self.0
        )
    }
}

impl std::error::Error for OutputFormatParseError {}
