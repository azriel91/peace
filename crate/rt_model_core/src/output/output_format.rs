use std::str::FromStr;

use crate::output::OutputFormatParseError;

/// How to format command output -- human readable or machine parsable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    /// Human readable text.
    Text,
    /// The YAML Ain't Markup Language™ ([YAML]) format.
    ///
    /// [YAML]: https://yaml.org/
    Yaml,
    /// The JavaScript Object Notation ([JSON]) format
    ///
    /// [JSON]: https://www.json.org/
    #[cfg(feature = "output_json")]
    Json,
}

impl FromStr for OutputFormat {
    type Err = OutputFormatParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(Self::Text),
            "yaml" => Ok(Self::Yaml),
            #[cfg(feature = "output_json")]
            "json" => Ok(Self::Json),
            _ => Err(OutputFormatParseError(s.to_string())),
        }
    }
}
