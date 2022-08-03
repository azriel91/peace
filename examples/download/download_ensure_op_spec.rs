use std::path::{Path, PathBuf};

use bytes::Bytes;
use futures::{Stream, StreamExt, TryStreamExt};
#[nougat::gat(Data)]
use peace::cfg::EnsureOpSpec;
use peace::{
    cfg::{async_trait, nougat, OpCheckStatus, ProgressLimit, State},
    diff::Tracked,
};
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

use crate::{DownloadError, DownloadParams, FileState, FileStateDiff};

/// Ensure OpSpec for the file to download.
#[derive(Debug)]
pub struct DownloadEnsureOpSpec;

impl DownloadEnsureOpSpec {
    async fn file_download(download_params: &DownloadParams<'_>) -> Result<(), DownloadError> {
        let client = download_params.client();
        let src_url = download_params.src();
        let dest = download_params.dest();
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
#[nougat::gat]
impl EnsureOpSpec for DownloadEnsureOpSpec {
    type Data<'op> = DownloadParams<'op>
        where Self: 'op;
    type Error = DownloadError;
    type StateDiff = FileStateDiff;
    type StateLogical = Option<FileState>;
    type StatePhysical = PathBuf;

    async fn check(
        _download_params: DownloadParams<'_>,
        _file_state_current: &State<Option<FileState>, PathBuf>,
        _file_state_desired: &Option<FileState>,
        diff: &FileStateDiff,
    ) -> Result<OpCheckStatus, DownloadError> {
        let op_check_status = match diff {
            FileStateDiff::Change {
                byte_len,
                contents: _,
            } => {
                let progress_limit = match byte_len.to {
                    Tracked::None => ProgressLimit::Unknown,
                    Tracked::Known(len) => len
                        .try_into()
                        .map(|len| ProgressLimit::Bytes(len))
                        .unwrap_or(ProgressLimit::Unknown),
                    Tracked::Unknown => ProgressLimit::Unknown,
                };

                OpCheckStatus::ExecRequired { progress_limit }
            }
            FileStateDiff::Deleted => OpCheckStatus::ExecNotRequired, // Don't delete existing file
            FileStateDiff::NoChangeNonExistent | FileStateDiff::NoChangeSync => {
                OpCheckStatus::ExecNotRequired
            }
        };
        Ok(op_check_status)
    }

    async fn exec_dry(
        download_params: DownloadParams<'_>,
        _state: &State<Option<FileState>, PathBuf>,
        _file_state_desired: &Option<FileState>,
        _diff: &FileStateDiff,
    ) -> Result<PathBuf, DownloadError> {
        let dest = download_params.dest();
        Ok(dest.to_path_buf())
    }

    async fn exec(
        download_params: DownloadParams<'_>,
        _state: &State<Option<FileState>, PathBuf>,
        _file_state_desired: &Option<FileState>,
        _diff: &FileStateDiff,
    ) -> Result<PathBuf, DownloadError> {
        Self::file_download(&download_params).await?;
        let dest = download_params.dest();
        Ok(dest.to_path_buf())
    }
}
