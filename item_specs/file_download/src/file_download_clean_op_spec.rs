use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, state::FetchedOpt, CleanOpSpec, OpCheckStatus, State};

use crate::{ETag, FileDownloadData, FileDownloadError, FileDownloadState};

/// `CleanOpSpec` for the file to download.
#[derive(Debug, Default)]
pub struct FileDownloadCleanOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> CleanOpSpec for FileDownloadCleanOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = FileDownloadData<'op, Id>;
    type Error = FileDownloadError;
    type State = State<FileDownloadState, FetchedOpt<ETag>>;

    async fn check(
        _file_download_data: FileDownloadData<'_, Id>,
        State {
            logical: file_state,
            physical: _e_tag,
        }: &State<FileDownloadState, FetchedOpt<ETag>>,
    ) -> Result<OpCheckStatus, FileDownloadError> {
        let op_check_status = match file_state {
            FileDownloadState::None { .. } => OpCheckStatus::ExecNotRequired,
            FileDownloadState::StringContents {
                path: _,
                #[cfg(not(feature = "output_progress"))]
                    contents: _,
                #[cfg(feature = "output_progress")]
                contents,
            } => {
                #[cfg(not(feature = "output_progress"))]
                {
                    OpCheckStatus::ExecRequired
                }
                #[cfg(feature = "output_progress")]
                {
                    OpCheckStatus::ExecRequired {
                        progress_limit: ProgressLimit::Bytes(
                            contents.as_bytes().len().try_into().unwrap(),
                        ),
                    }
                }
            }
            FileDownloadState::Length {
                path: _,
                #[cfg(not(feature = "output_progress"))]
                    byte_count: _,
                #[cfg(feature = "output_progress")]
                byte_count,
            } => {
                #[cfg(not(feature = "output_progress"))]
                {
                    OpCheckStatus::ExecRequired
                }

                #[cfg(feature = "output_progress")]
                OpCheckStatus::ExecRequired {
                    progress_limit: ProgressLimit::Bytes(*byte_count),
                }
            }
            FileDownloadState::Unknown { path: _ } => {
                #[cfg(not(feature = "output_progress"))]
                {
                    OpCheckStatus::ExecRequired
                }

                #[cfg(feature = "output_progress")]
                OpCheckStatus::ExecRequired {
                    progress_limit: ProgressLimit::Unknown,
                }
            }
        };
        Ok(op_check_status)
    }

    async fn exec_dry(
        _file_download_data: FileDownloadData<'_, Id>,
        _state: &State<FileDownloadState, FetchedOpt<ETag>>,
    ) -> Result<(), FileDownloadError> {
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn exec(
        _file_download_data: FileDownloadData<'_, Id>,
        State {
            logical: file_state,
            physical: _e_tag,
        }: &State<FileDownloadState, FetchedOpt<ETag>>,
    ) -> Result<(), FileDownloadError> {
        match file_state {
            FileDownloadState::None { .. } => {}
            FileDownloadState::StringContents { path, .. }
            | FileDownloadState::Length { path, .. }
            | FileDownloadState::Unknown { path } => {
                tokio::fs::remove_file(path)
                    .await
                    .map_err(FileDownloadError::DestFileRemove)?;
            }
        }
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    async fn exec(
        file_download_data: FileDownloadData<'_, Id>,
        State {
            logical: file_state,
            physical: e_tag,
        }: &State<FileDownloadState, FetchedOpt<ETag>>,
    ) -> Result<(), FileDownloadError> {
        match file_state {
            FileDownloadState::None { .. } => {}
            FileDownloadState::StringContents { path, .. }
            | FileDownloadState::Unknown { path } => {
                file_download_data.storage().remove_item(path)?;
            }
        }

        Ok(())
    }
}
