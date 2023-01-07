/// Where to output progress updates to -- stdout or stderr.
///
/// This defaults to `stderr`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CliOutputTarget {
    /// Standard error -- file descriptor 1.
    Stdout,
    /// Standard error -- file descriptor 2.
    Stderr,
}

impl Default for CliOutputTarget {
    fn default() -> Self {
        Self::Stderr
    }
}
