use std::path::{Path, PathBuf};

use bytes::Bytes;
use futures::{Stream, StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
};
use url::Url;
use zzzz::cfg::{
    async_trait::async_trait, diff::Diff, OpCheckStatus, OpSpec, OpSpecDry, ProgressLimit, WorkSpec,
};

fn main() {
    //
}

#[derive(Debug)]
pub struct Download;

impl Download {
    /// Read up to 1 kB in memory.
    pub const IN_MEMORY_CONTENTS_MAX: u64 = 1024;
}

#[derive(Debug)]
pub struct DownloadParams {
    /// Url of the file to download.
    src: Url,
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
    /// Length of the file.
    ///
    /// Use this when:
    ///
    /// * File is not practical to load in memory.
    Length(u64),
    /// Cannot determine file state.
    ///
    /// May be used for the desired state
    Unknown,
}

#[derive(Debug)]
pub struct DownloadStatusSpec;

#[async_trait]
impl OpSpec for DownloadStatusSpec {
    type Data = DownloadParams;
    type Error = DownloadError;
    type Output = Option<FileState>;
    type State = ();

    async fn setup(_download_params: &DownloadParams) -> Result<ProgressLimit, DownloadError> {
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
        let mut file = File::open(&download_params.dest)
            .await
            .map_err(DownloadError::DestFileOpen)?;
        let metadata = file
            .metadata()
            .await
            .map_err(DownloadError::DestMetadataRead)?;

        let state = if metadata.len() > Download::IN_MEMORY_CONTENTS_MAX {
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

#[derive(Debug)]
pub struct DownloadEnsureSpec;

impl DownloadEnsureSpec {
    async fn file_contents_check(
        download_params: &DownloadParams,
        client: &reqwest::Client,
        file_state_current: &FileState,
    ) -> Result<OpCheckStatus, DownloadError> {
        let file_state_desired = Self::file_state_desired(download_params, client).await?;

        let file_state_diff = file_state_current.diff(&file_state_desired);
        match file_state_diff {
            FileStateDiff::NoChange => Ok(OpCheckStatus::ExecNotRequired),
            FileStateDiff::StringContents(_)
            | FileStateDiff::Length(_)
            | FileStateDiff::Unknown => Ok(OpCheckStatus::ExecRequired),
        }
    }

    async fn file_state_desired(
        download_params: &DownloadParams,
        client: &reqwest::Client,
    ) -> Result<FileState, DownloadError> {
        let response = client
            .get(download_params.src.clone())
            .send()
            .await
            .map_err(DownloadError::SrcGet)?;

        let status_code = response.status();
        if status_code.is_success() {
            if let Some(remote_file_length) = response.content_length() {
                if remote_file_length <= Download::IN_MEMORY_CONTENTS_MAX {
                    // Download it now.
                    let remote_contents =
                        response.text().await.map_err(DownloadError::SrcFileRead)?;
                    Ok(FileState::StringContents(remote_contents))
                } else {
                    // Stream it later.
                    Ok(FileState::Length(remote_file_length))
                }
            } else {
                Ok(FileState::Unknown)
            }
        } else {
            Err(DownloadError::SrcFileUndetermined { status_code })
        }
    }

    async fn file_download(
        download_params: &DownloadParams,
        client: &reqwest::Client,
    ) -> Result<(), DownloadError> {
        let response = client
            .get(download_params.src.clone())
            .send()
            .await
            .map_err(DownloadError::SrcGet)?;

        Self::stream_write(&download_params.dest, response.bytes_stream()).await
    }

    /// Streams the content to disk.
    async fn stream_write(
        dest_path: &Path,
        byte_stream: impl Stream<Item = reqwest::Result<Bytes>>,
    ) -> Result<(), DownloadError> {
        let dest_file = File::create(dest_path)
            .await
            .map_err(DownloadError::DestFileCreate)?;

        let buffer = BufWriter::new(dest_file);
        let mut buffer = byte_stream
            .map(|bytes_result| bytes_result.map_err(DownloadError::ResponseBytesStream))
            .try_fold(buffer, |mut buffer, bytes| async move {
                // TODO: increment progress by bytes.len()
                buffer
                    .write_all(&bytes)
                    .await
                    .map_err(DownloadError::ResponseFileWrite)?;

                Ok(buffer)
            })
            .await?;
        buffer
            .flush()
            .await
            .map_err(DownloadError::ResponseFileWrite)?;
        Ok(())
    }
}

#[async_trait]
impl OpSpec for DownloadEnsureSpec {
    type Data = DownloadParams;
    type Error = DownloadError;
    type Output = PathBuf;
    type State = Option<FileState>;

    async fn setup(_download_params: &DownloadParams) -> Result<ProgressLimit, DownloadError> {
        // TODO: pass through desired State,
        Ok(ProgressLimit::Bytes(1024))
    }

    async fn check(
        download_params: &DownloadParams,
        file_state: &Option<FileState>,
    ) -> Result<OpCheckStatus, DownloadError> {
        let op_check_status = match file_state.as_ref() {
            Some(file_state) => {
                // TODO: the client should be part of Data.
                let client = reqwest::Client::new();
                let client = &client;
                Self::file_contents_check(download_params, client, file_state).await?
            }
            None => OpCheckStatus::ExecRequired,
        };
        Ok(op_check_status)
    }

    async fn exec(download_params: &DownloadParams) -> Result<PathBuf, DownloadError> {
        // TODO: the client should be part of Data.
        let client = reqwest::Client::new();
        let client = &client;

        Self::file_download(download_params, client).await?;
        Ok(download_params.dest.clone())
    }
}

#[async_trait]
impl OpSpecDry for DownloadEnsureSpec {
    async fn exec_dry() -> Result<Self::Output, Self::Error> {
        todo!("should this be inferred from the Diff instead")
    }
}

#[derive(Debug)]
pub struct DownloadCleanSpec;

#[async_trait]
impl OpSpec for DownloadCleanSpec {
    type Data = DownloadParams;
    type Error = DownloadError;
    type Output = PathBuf;
    type State = Option<FileState>;

    async fn setup(_download_params: &DownloadParams) -> Result<ProgressLimit, DownloadError> {
        // TODO: pass through desired State,

        // Bytes to delete
        Ok(ProgressLimit::Bytes(1024))
    }

    async fn check(
        _download_params: &DownloadParams,
        file_state: &Option<FileState>,
    ) -> Result<OpCheckStatus, DownloadError> {
        let op_check_status = if file_state.is_some() {
            OpCheckStatus::ExecRequired
        } else {
            OpCheckStatus::ExecNotRequired
        };
        Ok(op_check_status)
    }

    async fn exec(download_params: &DownloadParams) -> Result<PathBuf, DownloadError> {
        tokio::fs::remove_file(&download_params.dest)
            .await
            .map_err(DownloadError::DestFileRemove)?;
        Ok(download_params.dest.clone())
    }
}

#[async_trait]
impl OpSpecDry for DownloadCleanSpec {
    async fn exec_dry() -> Result<Self::Output, Self::Error> {
        todo!("should this be inferred from the Diff instead")
    }
}

impl WorkSpec for Download {
    type CleanOpSpec = DownloadCleanSpec;
    type EnsureOpSpec = DownloadEnsureSpec;
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
    #[error("Failed to open destination file for writing.")]
    DestFileCreate(std::io::Error),
    #[error("Failed to delete destination file.")]
    DestFileRemove(std::io::Error),
    #[error("Failed to parse source URL.")]
    SrcUrlParse(url::ParseError),
    #[error("Failed to parse source URL.")]
    SrcGet(reqwest::Error),
    #[error("Failed to fetch source file metadata. Response status code: {status_code}")]
    SrcFileUndetermined { status_code: reqwest::StatusCode },
    #[error("Failed to read source file content.")]
    SrcFileRead(reqwest::Error),
    #[error("Failed to stream source file content.")]
    ResponseBytesStream(reqwest::Error),
    #[error("Failed to transfer source file content.")]
    ResponseFileWrite(std::io::Error),
}
