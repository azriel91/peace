use std::path::PathBuf;

use peace::cfg::WorkSpec;

use crate::{DownloadCleanSpec, DownloadEnsureSpec, DownloadStatusSpec, FileState};

/// Work spec for downloading a file.
#[derive(Debug)]
pub struct DownloadWorkSpec;

impl WorkSpec for DownloadWorkSpec {
    type CleanOpSpec = DownloadCleanSpec;
    type EnsureOpSpec = DownloadEnsureSpec;
    type ResIds = PathBuf;
    type State = Option<FileState>;
    type StatusSpec = DownloadStatusSpec;
}
