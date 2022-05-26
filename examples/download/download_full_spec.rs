use std::path::PathBuf;

use peace::cfg::FullSpec;

use crate::{DownloadCleanSpec, DownloadEnsureSpec, DownloadStatusSpec, FileState};

/// Full spec for downloading a file.
#[derive(Debug)]
pub struct DownloadFullSpec;

impl<'op> FullSpec<'op> for DownloadFullSpec {
    type CleanOpSpec = DownloadCleanSpec;
    type EnsureOpSpec = DownloadEnsureSpec;
    type ResIds = PathBuf;
    type State = Option<FileState>;
    type StatusSpec = DownloadStatusSpec;
}
