use std::path::{Path, PathBuf};

use bytes::Bytes;
use futures::{Stream, StreamExt, TryStreamExt};
use peace::cfg::{async_trait, diff::Diff, EnsureOpSpec, OpCheckStatus, ProgressLimit};
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

use crate::{DownloadError, DownloadParams, FileState, FileStateDiff};

/// Ensure OpSpec for the file to download.
#[derive(Debug, Default)]
pub struct DownloadEnsureOpSpec;

impl DownloadEnsureOpSpec {
    async fn file_state_desired(
        download_params: &DownloadParams<'_>,
    ) -> Result<FileState, DownloadError> {
        let client = download_params.client();
        let src_url = download_params.src().ok_or(DownloadError::SrcUrlInit)?;
        let response = client
            .get(src_url.clone())
            .send()
            .await
            .map_err(DownloadError::SrcGet)?;

        let status_code = response.status();
        if status_code.is_success() {
            if let Some(remote_file_length) = response.content_length() {
                if remote_file_length <= crate::IN_MEMORY_CONTENTS_MAX {
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

    async fn file_contents_check(
        _download_params: &DownloadParams<'_>,
        file_state_current: &FileState,
        file_state_desired: &FileState,
    ) -> Result<OpCheckStatus, DownloadError> {
        let file_state_diff = file_state_current.diff(&file_state_desired);
        match file_state_diff {
            FileStateDiff::NoChange => Ok(OpCheckStatus::ExecNotRequired),
            FileStateDiff::StringContents(_)
            | FileStateDiff::Length(_)
            | FileStateDiff::Unknown => Ok(OpCheckStatus::ExecRequired {
                progress_limit: ProgressLimit::Bytes(1024),
            }),
        }
    }

    async fn file_download(download_params: &DownloadParams<'_>) -> Result<(), DownloadError> {
        let client = download_params.client();
        let src_url = download_params.src().ok_or(DownloadError::SrcUrlInit)?;
        let dest = download_params.dest().ok_or(DownloadError::DestFileInit)?;
        let response = client
            .get(src_url.clone())
            .send()
            .await
            .map_err(DownloadError::SrcGet)?;

        Self::stream_write(dest, response.bytes_stream()).await
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
impl<'op> EnsureOpSpec<'op> for DownloadEnsureOpSpec {
    type Data = DownloadParams<'op>;
    type Error = DownloadError;
    type StateLogical = Option<FileState>;
    type StatePhysical = PathBuf;

    async fn desired(
        download_params: DownloadParams<'op>,
    ) -> Result<Option<FileState>, DownloadError> {
        let file_state_desired = Self::file_state_desired(&download_params).await?;

        Ok(Some(file_state_desired))
    }

    async fn check(
        download_params: DownloadParams<'op>,
        file_state_current: &Option<FileState>,
        file_state_desired: &Option<FileState>,
    ) -> Result<OpCheckStatus, DownloadError> {
        let op_check_status = match (file_state_current.as_ref(), file_state_desired.as_ref()) {
            (Some(file_state_current), Some(file_state_desired)) => {
                Self::file_contents_check(&download_params, file_state_current, file_state_desired)
                    .await?
            }
            (Some(_file_state_current), None) => {
                // Should we delete the file?
                OpCheckStatus::ExecNotRequired
            }
            (None, Some(file_state_desired)) => {
                let progress_limit = match file_state_desired {
                    FileState::StringContents(s) => TryInto::<u64>::try_into(s.bytes().len())
                        .map(ProgressLimit::Bytes)
                        .unwrap_or(ProgressLimit::Unknown),
                    FileState::Length(len) => ProgressLimit::Bytes(*len),
                    FileState::Unknown => ProgressLimit::Unknown,
                };

                OpCheckStatus::ExecRequired { progress_limit }
            }
            (None, None) => OpCheckStatus::ExecNotRequired,
        };
        Ok(op_check_status)
    }

    async fn exec_dry(
        download_params: DownloadParams<'op>,
        _file_state_current: &Option<FileState>,
        _file_state_desired: &Option<FileState>,
    ) -> Result<PathBuf, DownloadError> {
        let dest = download_params.dest().ok_or(DownloadError::DestFileInit)?;
        Ok(dest.to_path_buf())
    }

    async fn exec(
        download_params: DownloadParams<'op>,
        _file_state_current: &Option<FileState>,
        _file_state_desired: &Option<FileState>,
    ) -> Result<PathBuf, DownloadError> {
        Self::file_download(&download_params).await?;
        let dest = download_params.dest().ok_or(DownloadError::DestFileInit)?;
        Ok(dest.to_path_buf())
    }
}
