#[nougat::gat(Data)]
use peace::cfg::CleanOpSpec;
use peace::cfg::{async_trait, nougat, state::Nothing, OpCheckStatus, ProgressLimit, State};

use crate::{FileDownloadData, FileDownloadError, FileDownloadState};

/// `CleanOpSpec` for the file to download.
#[derive(Debug)]
pub struct FileDownloadCleanOpSpec;

#[async_trait(?Send)]
#[nougat::gat]
impl CleanOpSpec for FileDownloadCleanOpSpec {
    type Data<'op> = FileDownloadData<'op>
        where Self: 'op;
    type Error = FileDownloadError;
    type StateLogical = FileDownloadState;
    type StatePhysical = Nothing;

    async fn check(
        _file_download_data: FileDownloadData<'_>,
        State {
            logical: file_state,
            ..
        }: &State<FileDownloadState, Nothing>,
    ) -> Result<OpCheckStatus, FileDownloadError> {
        let op_check_status = match file_state {
            FileDownloadState::None { .. } => OpCheckStatus::ExecNotRequired,
            FileDownloadState::StringContents { path: _, contents } => {
                OpCheckStatus::ExecRequired {
                    progress_limit: ProgressLimit::Bytes(
                        contents.as_bytes().len().try_into().unwrap(),
                    ),
                }
            }
            FileDownloadState::Length {
                path: _,
                byte_count,
            } => OpCheckStatus::ExecRequired {
                progress_limit: ProgressLimit::Bytes(*byte_count),
            },
            FileDownloadState::Unknown { path: _ } => OpCheckStatus::ExecRequired {
                progress_limit: ProgressLimit::Unknown,
            },
        };
        Ok(op_check_status)
    }

    async fn exec_dry(
        _file_download_data: FileDownloadData<'_>,
        _state: &State<FileDownloadState, Nothing>,
    ) -> Result<(), FileDownloadError> {
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn exec(
        _file_download_data: FileDownloadData<'_>,
        State {
            logical: file_state,
            ..
        }: &State<FileDownloadState, Nothing>,
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
        file_download_data: FileDownloadData<'_>,
        State {
            logical: file_state,
            ..
        }: &State<FileDownloadState, Nothing>,
    ) -> Result<(), FileDownloadError> {
        match file_state {
            FileDownloadState::None { .. } => {}
            FileDownloadState::StringContents { path, .. }
            | FileDownloadState::Length { path, .. }
            | FileDownloadState::Unknown { path } => {
                file_download_data.storage().remove_item(path)?;
            }
        }

        Ok(())
    }
}
