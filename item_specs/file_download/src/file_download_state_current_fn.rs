use std::marker::PhantomData;

use peace::cfg::{state::FetchedOpt, FnCtx, State};
#[cfg(not(target_arch = "wasm32"))]
use tokio::{fs::File, io::AsyncReadExt};

#[cfg(target_arch = "wasm32")]
use peace::rt_model::Storage;

use crate::{ETag, FileDownloadData, FileDownloadError, FileDownloadState};

/// Reads the current state of the file to download.
#[derive(Debug)]
pub struct FileDownloadStateCurrentFn<Id>(PhantomData<Id>);

impl<Id> FileDownloadStateCurrentFn<Id>
where
    Id: Send + Sync,
{
    pub async fn try_state_current(
        fn_ctx: FnCtx<'_>,
        data: FileDownloadData<'_, Id>,
    ) -> Result<Option<State<FileDownloadState, FetchedOpt<ETag>>>, FileDownloadError> {
        Self::state_current(fn_ctx, data).await.map(Some)
    }

    pub async fn state_current(
        _fn_ctx: FnCtx<'_>,
        data: FileDownloadData<'_, Id>,
    ) -> Result<State<FileDownloadState, FetchedOpt<ETag>>, FileDownloadError> {
        let dest = data.file_download_params().dest();

        #[cfg(not(target_arch = "wasm32"))]
        let file_exists = dest.exists();
        #[cfg(target_arch = "wasm32")]
        let file_exists = data.storage().get_item_opt(dest)?.is_some();
        if !file_exists {
            let path = dest.to_path_buf();
            return Ok(State::new(
                FileDownloadState::None { path },
                FetchedOpt::Tbd,
            ));
        }

        // Check file length
        #[cfg(not(target_arch = "wasm32"))]
        let file_state = Self::read_file_contents(dest).await?;

        #[cfg(target_arch = "wasm32")]
        let file_state = Self::read_file_contents(dest, data.storage()).await?;

        let e_tag = data
            .state_working()
            .as_ref()
            .map(|state_working| state_working.physical.clone())
            .or_else(|| {
                data.state_prev()
                    .get()
                    .map(|state_prev| state_prev.physical.clone())
            })
            .unwrap_or(if let FileDownloadState::None { .. } = &file_state {
                FetchedOpt::Tbd
            } else {
                FetchedOpt::None
            });

        Ok(State::new(file_state, e_tag))
    }

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
            FileDownloadState::Unknown {
                path: dest.to_path_buf(),
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
                    .map(|byte_count: u64| {
                        if byte_count > crate::IN_MEMORY_CONTENTS_MAX {
                            FileDownloadState::Unknown {
                                path: dest.to_path_buf(),
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
