use std::str::FromStr;

use crate::output::CliColorizeParseError;

/// Whether to colourize output using ANSI codes on the CLI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CliColorize {
    /// Automatically detect whether to colourize the output.
    Auto,
    /// Always colourize the output.
    Always,
    /// Never colourize the output.
    Never,
}

impl FromStr for CliColorize {
    type Err = CliColorizeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            _ => Err(CliColorizeParseError(s.to_string())),
        }
    }
}

/// Whether or not to render output with ANSI colour codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CliColorizeUsed {
    /// Render the output with ANSI colour codes.
    Colored,
    /// Render the output without ANSI colour codes.
    Uncolored,
}
