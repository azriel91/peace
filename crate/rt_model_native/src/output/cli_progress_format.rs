/// How to format progress on the CLI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CliProgressFormat {
    /// Render progress in the same format as the outcome.
    Outcome,
    /// Always render progress as a progress bar.
    ProgressBar,
    /// Don't render progress.
    None,
}
