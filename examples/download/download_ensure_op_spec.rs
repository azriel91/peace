use std::path::{Path, PathBuf};

use bytes::Bytes;
use futures::{Stream, StreamExt, TryStreamExt};
use peace::{
    cfg::{async_trait, diff::Diff, OpCheckStatus, OpSpec, OpSpecDry, ProgressLimit},
    data::Resources,
};
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

use crate::{DownloadError, DownloadParams, FileState, FileStateDiff};

/// Ensure OpSpec for the file to download.
#[derive(Debug, Default)]
pub struct DownloadEnsureOpSpec;

impl DownloadEnsureOpSpec {
    async fn file_contents_check(
        download_params: &DownloadParams<'_>,
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
        download_params: &DownloadParams<'_>,
        client: &reqwest::Client,
    ) -> Result<FileState, DownloadError> {
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

    async fn file_download(
        download_params: &DownloadParams<'_>,
        client: &reqwest::Client,
    ) -> Result<(), DownloadError> {
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
impl<'op> OpSpec<'op> for DownloadEnsureOpSpec {
    type Data = DownloadParams<'op>;
    type Error = DownloadError;
    type Output = PathBuf;
    type State = Option<FileState>;

    async fn setup(_resources: &mut Resources) -> Result<ProgressLimit, DownloadError> {
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
        let dest = download_params.dest().ok_or(DownloadError::DestFileInit)?;
        Ok(dest.to_path_buf())
    }
}

#[async_trait]
impl<'op> OpSpecDry<'op> for DownloadEnsureOpSpec {
    async fn exec_dry() -> Result<Self::Output, Self::Error> {
        todo!("should this be inferred from the Diff instead")
    }
}
