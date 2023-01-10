/// How to format progress on the CLI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CliProgressFormatUsed {
    /// Render progress in the same format as the output.
    Output,
    /// Always render progress as a progress bar.
    ProgressBar,
}
