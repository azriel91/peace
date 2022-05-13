/// Unit of measurement and limit to indicate progress.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProgressLimit {
    /// There is no meaningful way to measure progress.
    Unknown,
    /// Progress is complete when `n` steps have been completed.
    Steps(u64),
    /// Progress is complete when `n` bytes are processed.
    ///
    /// Useful for upload / download progress.
    Bytes(u64),
}

impl Default for ProgressLimit {
    fn default() -> Self {
        Self::Unknown
    }
}
