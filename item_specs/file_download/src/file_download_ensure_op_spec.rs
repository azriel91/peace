use std::marker::PhantomData;
#[cfg(target_arch = "wasm32")]
use std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
use bytes::Bytes;
#[cfg(not(target_arch = "wasm32"))]
use futures::{Stream, StreamExt, TryStreamExt};
use peace::cfg::{state::Nothing, OpCtx};
#[cfg(not(target_arch = "wasm32"))]
use tokio::io::AsyncWriteExt;
#[cfg(not(target_arch = "wasm32"))]
use tokio::{fs::File, io::BufWriter};

#[cfg(target_arch = "wasm32")]
use peace::rt_model::Storage;

use peace::{
    cfg::{async_trait, EnsureOpSpec, OpCheckStatus, ProgressLimit, ProgressUpdate, Sender, State},
    diff::Tracked,
};

#[cfg(not(target_arch = "wasm32"))]
use crate::FileDownloadParams;
use crate::{FileDownloadData, FileDownloadError, FileDownloadState, FileDownloadStateDiff};

/// Ensure OpSpec for the file to download.
#[derive(Debug)]
pub struct FileDownloadEnsureOpSpec<Id>(PhantomData<Id>);

impl<Id> FileDownloadEnsureOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    async fn file_download(
        op_ctx: OpCtx<'_>,
        file_download_data: FileDownloadData<'_, Id>,
    ) -> Result<(), FileDownloadError> {
        let client = file_download_data.client();
        let params = file_download_data.file_download_params();
        let src_url = params.src();
        let dest = params.dest();
        let response = client.get(src_url.clone()).send().await.map_err(|error| {
            #[cfg(not(target_arch = "wasm32"))]
            let (Ok(file_download_error) | Err(file_download_error)) =
                FileDownloadError::src_get(src_url.clone(), dest, error);
            #[cfg(target_arch = "wasm32")]
            let file_download_error = FileDownloadError::src_get(src_url.clone(), error);

            file_download_error
        })?;

        #[cfg(not(target_arch = "wasm32"))]
        {
            Self::stream_write(op_ctx.progress_tx, params, response.bytes_stream()).await?;
        }

        // reqwest in wasm doesn't support streams
        // https://github.com/seanmonstar/reqwest/issues/1424
        #[cfg(target_arch = "wasm32")]
        {
            Self::stream_write(
                op_ctx.progress_tx,
                dest,
                file_download_data.storage(),
                params.storage_form(),
                response,
            )
            .await?;
        }

        Ok(())
    }

    /// Streams the content to disk.
    #[cfg(not(target_arch = "wasm32"))]
    async fn stream_write(
        progress_tx: &Sender<ProgressUpdate>,
        file_download_params: &FileDownloadParams<Id>,
        byte_stream: impl Stream<Item = reqwest::Result<Bytes>>,
    ) -> Result<(), FileDownloadError> {
        use std::{fmt::Write, path::Component};

        #[cfg(feature = "error_reporting")]
        use peace::miette::{SourceOffset, SourceSpan};

        let dest_path = file_download_params.dest();
        if let Some(dest_parent) = dest_path.parent() {
            // Ensure all parent directories are created
            tokio::fs::create_dir_all(dest_parent)
                .await
                .map_err(|error| {
                    #[cfg(feature = "error_reporting")]
                    let dest_display = format!("{}", dest_path.display());
                    #[cfg(feature = "error_reporting")]
                    let parent_dirs_span = {
                        let start = 1usize;
                        SourceSpan::from((start, dest_display.len()))
                    };

                    FileDownloadError::DestParentDirsCreate {
                        dest: dest_path.to_path_buf(),
                        dest_parent: dest_parent.to_path_buf(),
                        #[cfg(feature = "error_reporting")]
                        dest_display,
                        #[cfg(feature = "error_reporting")]
                        parent_dirs_span,
                        error,
                    }
                })?;
        }
        let dest_file = File::create(dest_path).await.or_else(|error| {
            let mut init_command_approx = String::with_capacity(256);
            let exe_path = std::env::current_exe().map_err(FileDownloadError::CurrentExeRead)?;
            let exe_name =
                if let Some(Component::Normal(exe_name)) = exe_path.components().next_back() {
                    exe_name
                } else {
                    return Err(FileDownloadError::CurrentExeNameRead);
                };

            let exe_name = exe_name.to_string_lossy();
            let src = file_download_params.src();
            let dest = dest_path.to_path_buf();
            let dest_display = dest.display();

            write!(&mut init_command_approx, "{exe_name} init {src} ")
                .map_err(FileDownloadError::FormatString)?;
            #[cfg(feature = "error_reporting")]
            let dest_offset_col = init_command_approx.len();
            write!(&mut init_command_approx, "{dest_display}")
                .map_err(FileDownloadError::FormatString)?;

            #[cfg(feature = "error_reporting")]
            let dest_span = {
                let loc_line = 1;
                // Add one to offset because we are 1-based, not 0-based?
                let start = SourceOffset::from_location(
                    &init_command_approx,
                    loc_line,
                    dest_offset_col + 1,
                );
                // Add one to length because we are 1-based, not 0-based?
                let length = SourceOffset::from_location(
                    &init_command_approx,
                    loc_line,
                    init_command_approx.len() - dest_offset_col + 1,
                );
                SourceSpan::new(start, length)
            };
            Err(FileDownloadError::DestFileCreate {
                init_command_approx,
                #[cfg(feature = "error_reporting")]
                dest_span,
                dest,
                error,
            })
        })?;

        let buffer = BufWriter::new(dest_file);
        let mut buffer = byte_stream
            .map(|bytes_result| bytes_result.map_err(FileDownloadError::ResponseBytesStream))
            .try_fold(buffer, |mut buffer, bytes| async move {
                buffer
                    .write_all(&bytes)
                    .await
                    .map_err(FileDownloadError::ResponseFileWrite)?;

                let _progress_send = {
                    let progress_update = if let Ok(progress_inc) = u64::try_from(bytes.len()) {
                        ProgressUpdate::Inc(progress_inc)
                    } else {
                        ProgressUpdate::Tick
                    };
                    progress_tx.send(progress_update).await
                };

                Ok(buffer)
            })
            .await?;
        buffer
            .flush()
            .await
            .map_err(FileDownloadError::ResponseFileWrite)?;
        Ok(())
    }

    /// Streams the content to disk.
    #[cfg(target_arch = "wasm32")]
    async fn stream_write(
        progress_tx: &Sender<ProgressUpdate>,
        dest_path: &Path,
        storage: &Storage,
        storage_form: crate::StorageForm,
        response: reqwest::Response,
    ) -> Result<(), FileDownloadError> {
        use crate::StorageForm;

        match storage_form {
            StorageForm::Text => {
                let value = response
                    .text()
                    .await
                    .map_err(FileDownloadError::ResponseTextRead)?;
                storage.set_item(dest_path, &value)?;
            }
            StorageForm::Base64 => {
                let bytes = response
                    .bytes()
                    .await
                    .map_err(FileDownloadError::ResponseBytesRead)?;
                storage.set_item_b64(dest_path, &bytes)?;
            }
        }

        Ok(())
    }
}

#[async_trait(?Send)]
impl<Id> EnsureOpSpec for FileDownloadEnsureOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = FileDownloadData<'op, Id>;
    type Error = FileDownloadError;
    type StateDiff = FileDownloadStateDiff;
    type StateLogical = FileDownloadState;
    type StatePhysical = Nothing;

    async fn check(
        _file_download_data: FileDownloadData<'_, Id>,
        _file_state_current: &State<FileDownloadState, Nothing>,
        _file_state_desired: &FileDownloadState,
        diff: &FileDownloadStateDiff,
    ) -> Result<OpCheckStatus, FileDownloadError> {
        let op_check_status = match diff {
            FileDownloadStateDiff::Change { byte_len, .. } => {
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
            FileDownloadStateDiff::Deleted { .. } => OpCheckStatus::ExecNotRequired, /* Don't delete */
            // existing file
            FileDownloadStateDiff::NoChangeNotExists { .. }
            | FileDownloadStateDiff::NoChangeSync { .. } => OpCheckStatus::ExecNotRequired,
        };
        Ok(op_check_status)
    }

    async fn exec_dry(
        _op_ctx: OpCtx<'_>,
        _file_download_data: FileDownloadData<'_, Id>,
        _state: &State<FileDownloadState, Nothing>,
        _file_state_desired: &FileDownloadState,
        _diff: &FileDownloadStateDiff,
    ) -> Result<Nothing, FileDownloadError> {
        Ok(Nothing)
    }

    async fn exec(
        op_ctx: OpCtx<'_>,
        file_download_data: FileDownloadData<'_, Id>,
        _state: &State<FileDownloadState, Nothing>,
        _file_state_desired: &FileDownloadState,
        _diff: &FileDownloadStateDiff,
    ) -> Result<Nothing, FileDownloadError> {
        Self::file_download(op_ctx, file_download_data).await?;
        Ok(Nothing)
    }
}
