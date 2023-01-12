use std::str::FromStr;

use crate::output::CliColorizeOptParseError;

/// Whether to colourize output using ANSI codes on the CLI.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CliColorizeOpt {
    /// Automatically detect whether to colourize the output.
    #[default]
    Auto,
    /// Always colourize the output.
    Always,
    /// Never colourize the output.
    Never,
}

impl FromStr for CliColorizeOpt {
    type Err = CliColorizeOptParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            _ => Err(CliColorizeOptParseError(s.to_string())),
        }
    }
}
