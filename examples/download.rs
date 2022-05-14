use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};
use zzzz::cfg::{
    async_trait::async_trait, diff::Diff, OpCheckStatus, OpSpec, ProgressLimit, WorkSpec,
};

fn main() {
    //
}

#[derive(Debug)]
pub struct Download;

#[derive(Debug)]
pub struct DownloadParams {
    /// Path of the file to download.
    ///
    /// Must be a file path, and not a directory.
    src: PathBuf,
    /// Path of the destination.
    ///
    /// Must be a file path, and not a directory.
    dest: PathBuf,
}

#[derive(Debug, Diff, Serialize, Deserialize, PartialEq)]
pub enum FileState {
    /// String contents of the file.
    ///
    /// Use this when:
    ///
    /// * File contents is text.
    /// * File is small enough to load in memory.
    StringContents(String),
    /// Binary contents of the file.
    ///
    /// Use this when:
    ///
    /// * File contents is binary.
    /// * File is small enough to load in memory.
    BinContents(Vec<u8>),
    /// Length of the file.
    ///
    /// Use this when:
    ///
    /// * File is not practical to load in memory.
    Length(u64),
}

#[derive(Debug)]
pub struct DownloadStatusSpec {}

#[async_trait]
impl OpSpec for DownloadStatusSpec {
    type Data = DownloadParams;
    type Error = DownloadError;
    type Output = Option<FileState>;
    type State = ();

    async fn setup(download_params: &DownloadParams) -> Result<ProgressLimit, DownloadError> {
        // Need to make one request.
        Ok(ProgressLimit::Steps(1))
    }

    async fn check(_: &DownloadParams, _: &()) -> Result<OpCheckStatus, DownloadError> {
        Ok(OpCheckStatus::ExecRequired)
    }

    async fn exec(download_params: &DownloadParams) -> Result<Option<FileState>, DownloadError> {
        // Destination file doesn't exist.
        if !download_params.dest.exists() {
            return Ok(None);
        }

        // Check file length
        let file = File::open(&download_params.dest)
            .await
            .map_err(DownloadError::DestFileOpen)?;
        let metadata = file
            .metadata()
            .await
            .map_err(DownloadError::DestMetadataRead)?;

        let state = if metadata.len() > 20 {
            Some(FileState::Length(metadata.len()))
        } else {
            let mut buffer = String::new();

            file.read_to_string(&mut buffer)
                .await
                .map_err(DownloadError::DestFileRead)?;
            Some(FileState::StringContents(buffer))
        };

        Ok(state)
    }
}

impl WorkSpec for Download {
    type CleanOpSpec = Type;
    type EnsureOpSpec = Type;
    type ResIds = PathBuf;
    type State = Option<FileState>;
    type StatusSpec = DownloadStatusSpec;
}

#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("Failed to open destination file.")]
    DestFileOpen(std::io::Error),
    #[error("Failed to read destination file metadata.")]
    DestMetadataRead(std::io::Error),
    #[error("Failed to read destination file contents.")]
    DestFileRead(std::io::Error),
}
