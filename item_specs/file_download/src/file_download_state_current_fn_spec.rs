use std::marker::PhantomData;

use peace::cfg::{async_trait, state::Nothing, State, TryFnSpec};
#[cfg(not(target_arch = "wasm32"))]
use tokio::{fs::File, io::AsyncReadExt};

#[cfg(target_arch = "wasm32")]
use peace::rt_model::Storage;

use crate::{FileDownloadData, FileDownloadError, FileDownloadState};

/// Reads the current state of the file to download.
#[derive(Debug)]
pub struct FileDownloadStateCurrentFnSpec<Id>(PhantomData<Id>);

impl<Id> FileDownloadStateCurrentFnSpec<Id> {
    #[cfg(not(target_arch = "wasm32"))]
    async fn read_file_contents(
        dest: &std::path::Path,
    ) -> Result<FileDownloadState, FileDownloadError> {
        let mut file = File::open(dest)
            .await
            .map_err(FileDownloadError::DestFileOpen)?;
        let metadata = file
            .metadata()
            .await
            .map_err(FileDownloadError::DestMetadataRead)?;
        let file_state = if metadata.len() > crate::IN_MEMORY_CONTENTS_MAX {
            FileDownloadState::Length {
                path: dest.to_path_buf(),
                byte_count: metadata.len(),
            }
        } else {
            let mut buffer = String::new();

            file.read_to_string(&mut buffer)
                .await
                .map_err(FileDownloadError::DestFileRead)?;
            FileDownloadState::StringContents {
                path: dest.to_path_buf(),
                contents: buffer,
            }
        };
        Ok(file_state)
    }

    #[cfg(target_arch = "wasm32")]
    async fn read_file_contents(
        dest: &std::path::Path,
        storage: &Storage,
    ) -> Result<FileDownloadState, FileDownloadError> {
        let file_state = storage
            .get_item_opt(dest)?
            .map(|contents| {
                contents
                    .bytes()
                    .len()
                    .try_into()
                    .map(|byte_count| {
                        if byte_count > crate::IN_MEMORY_CONTENTS_MAX {
                            FileDownloadState::Length {
                                path: dest.to_path_buf(),
                                byte_count,
                            }
                        } else {
                            FileDownloadState::StringContents {
                                path: dest.to_path_buf(),
                                contents: contents.clone(),
                            }
                        }
                    })
                    .unwrap_or_else(|_| FileDownloadState::StringContents {
                        path: dest.to_path_buf(),
                        contents: contents.clone(),
                    })
            })
            .unwrap_or(FileDownloadState::None {
                path: dest.to_path_buf(),
            });

        Ok(file_state)
    }
}

#[async_trait(?Send)]
impl<Id> TryFnSpec for FileDownloadStateCurrentFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = FileDownloadData<'op, Id>;
    type Error = FileDownloadError;
    type Output = State<FileDownloadState, Nothing>;

    async fn try_exec(
        file_download_data: FileDownloadData<'_, Id>,
    ) -> Result<Option<Self::Output>, FileDownloadError> {
        Self::exec(file_download_data).await.map(Some)
    }

    async fn exec(
        file_download_data: FileDownloadData<'_, Id>,
    ) -> Result<Self::Output, FileDownloadError> {
        let dest = file_download_data.file_download_params().dest();

        #[cfg(not(target_arch = "wasm32"))]
        let file_exists = dest.exists();
        #[cfg(target_arch = "wasm32")]
        let file_exists = file_download_data.storage().get_item_opt(dest)?.is_some();
        if !file_exists {
            let path = dest.to_path_buf();
            return Ok(State::new(FileDownloadState::None { path }, Nothing));
        }

        // Check file length
        #[cfg(not(target_arch = "wasm32"))]
        let file_state = Self::read_file_contents(dest).await?;

        #[cfg(target_arch = "wasm32")]
        let file_state = Self::read_file_contents(dest, file_download_data.storage()).await?;

        Ok(State::new(file_state, Nothing))
    }
}
