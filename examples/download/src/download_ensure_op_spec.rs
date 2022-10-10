use std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
use bytes::Bytes;
#[cfg(not(target_arch = "wasm32"))]
use futures::{Stream, StreamExt, TryStreamExt};
use peace::cfg::state::Nothing;
#[cfg(not(target_arch = "wasm32"))]
use tokio::io::AsyncWriteExt;
#[cfg(not(target_arch = "wasm32"))]
use tokio::{fs::File, io::BufWriter};

#[cfg(target_arch = "wasm32")]
use peace::rt_model::Storage;

#[nougat::gat(Data)]
use peace::cfg::EnsureOpSpec;
use peace::{
    cfg::{async_trait, nougat, OpCheckStatus, ProgressLimit, State},
    diff::Tracked,
};

use crate::{DownloadError, DownloadParams, FileState, FileStateDiff};

/// Ensure OpSpec for the file to download.
#[derive(Debug)]
pub struct DownloadEnsureOpSpec;

impl DownloadEnsureOpSpec {
    async fn file_download(download_params: DownloadParams<'_>) -> Result<(), DownloadError> {
        let client = download_params.client();
        let src_url = download_params.download_profile_init().src();
        let response = client
            .get(src_url.clone())
            .send()
            .await
            .map_err(DownloadError::SrcGet)?;

        #[cfg(not(target_arch = "wasm32"))]
        {
            Self::stream_write(
                download_params.download_profile_init().dest(),
                response.bytes_stream(),
            )
            .await?;
        }

        // reqwest in wasm doesn't support streams
        // https://github.com/seanmonstar/reqwest/issues/1424
        #[cfg(target_arch = "wasm32")]
        {
            let dest = download_params.download_profile_init().dest();
            Self::stream_write(dest, download_params.storage(), response).await?;
        }

        Ok(())
    }

    /// Streams the content to disk.
    #[cfg(not(target_arch = "wasm32"))]
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

    /// Streams the content to disk.
    #[cfg(target_arch = "wasm32")]
    async fn stream_write(
        dest_path: &Path,
        storage: &Storage,
        response: reqwest::Response,
    ) -> Result<(), DownloadError> {
        let response_text = response.text();
        let contents = response_text
            .await
            .map_err(DownloadError::ResponseTextRead)?;

        storage.set_item(dest_path, &contents)?;

        Ok(())
    }
}

#[async_trait(?Send)]
#[nougat::gat]
impl EnsureOpSpec for DownloadEnsureOpSpec {
    type Data<'op> = DownloadParams<'op>
        where Self: 'op;
    type Error = DownloadError;
    type StateDiff = FileStateDiff;
    type StateLogical = FileState;
    type StatePhysical = Nothing;

    async fn check(
        _download_params: DownloadParams<'_>,
        _file_state_current: &State<FileState, Nothing>,
        _file_state_desired: &FileState,
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
                        .map(ProgressLimit::Bytes)
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
        _download_params: DownloadParams<'_>,
        _state: &State<FileState, Nothing>,
        _file_state_desired: &FileState,
        _diff: &FileStateDiff,
    ) -> Result<Nothing, DownloadError> {
        Ok(Nothing)
    }

    async fn exec(
        download_params: DownloadParams<'_>,
        _state: &State<FileState, Nothing>,
        _file_state_desired: &FileState,
        _diff: &FileStateDiff,
    ) -> Result<Nothing, DownloadError> {
        Self::file_download(download_params).await?;
        Ok(Nothing)
    }
}
