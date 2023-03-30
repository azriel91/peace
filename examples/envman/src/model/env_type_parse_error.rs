use std::fmt;

/// Failed to parse environment type from string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnvTypeParseError(pub String);

impl fmt::Display for EnvTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"Failed to parse environment type from string: `"{}"`. Valid values are ["development", "production"]"#,
            self.0
        )
    }
}

impl std::error::Error for EnvTypeParseError {}
