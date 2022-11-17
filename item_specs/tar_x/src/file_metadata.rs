use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Metadata about a file in the tar.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FileMetadata {
    /// Path to the file, relative to either the tar root, or the extraction
    /// directory root.
    path: PathBuf,
    /// Last modification time of the file.
    ///
    /// Corresponds to [`mtime`] on Unix, and [`last_write_time`] on Windows.
    ///
    /// [`mtime`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html#method.mtime
    /// [`last_write_time`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html#method.last_write_time
    modified_time: u64,
}

impl From<tar::Header> for FileMetadata {
    fn from(_header: tar::Header) -> Self {
        todo!()
    }
}

impl From<std::fs::Metadata> for FileMetadata {
    fn from(_metadata: std::fs::Metadata) -> Self {
        todo!()
    }
}
