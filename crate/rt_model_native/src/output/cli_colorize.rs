/// Whether or not to render output with ANSI colour codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CliColorize {
    /// Render the output with ANSI colour codes.
    Colored,
    /// Render the output without ANSI colour codes.
    Uncolored,
}
