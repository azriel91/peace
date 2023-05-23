use std::fmt;

/// Failed to parse flow from string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnvManFlowParseError(pub String);

impl fmt::Display for EnvManFlowParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"Failed to parse flow from string: `"{}"`. Valid values are ["upload", "deploy"]"#,
            self.0
        )
    }
}

impl std::error::Error for EnvManFlowParseError {}
