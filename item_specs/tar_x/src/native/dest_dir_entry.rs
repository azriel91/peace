use std::path::PathBuf;

use tokio::fs::DirEntry;

/// Intermediary type while calculating `FileMetadata` for native targets.
#[derive(Debug)]
pub(crate) struct DestDirEntry {
    /// Path relative to the extraction directory.
    pub(crate) dest_dir_relative_path: PathBuf,
    /// `DirEntry` from `tokio`.
    pub(crate) dir_entry: DirEntry,
}
