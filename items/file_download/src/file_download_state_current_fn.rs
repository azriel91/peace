use std::{marker::PhantomData, path::Path};

use peace::{
    cfg::{state::FetchedOpt, FnCtx},
    params::Params,
};
#[cfg(not(target_arch = "wasm32"))]
use tokio::{fs::File, io::AsyncReadExt};

#[cfg(target_arch = "wasm32")]
use peace::rt_model::Storage;

use crate::{
    FileDownloadData, FileDownloadError, FileDownloadParams, FileDownloadState,
    FileDownloadStateLogical,
};

/// Reads the current state of the file to download.
#[derive(Debug)]
pub struct FileDownloadStateCurrentFn<Id>(PhantomData<Id>);

impl<Id> FileDownloadStateCurrentFn<Id>
where
    Id: Send + Sync,
{
    pub async fn try_state_current(
        _fn_ctx: FnCtx<'_>,
        params_partial: &<FileDownloadParams<Id> as Params>::Partial,
        data: FileDownloadData<'_, Id>,
    ) -> Result<Option<FileDownloadState>, FileDownloadError> {
        if let Some(dest) = params_partial.dest() {
            Self::state_current_internal(data, dest).await.map(Some)
        } else {
            Ok(None)
        }
    }

    pub async fn state_current(
        _fn_ctx: FnCtx<'_>,
        params: &FileDownloadParams<Id>,
        data: FileDownloadData<'_, Id>,
    ) -> Result<FileDownloadState, FileDownloadError> {
        let dest = params.dest();

        Self::state_current_internal(data, dest).await
    }

    async fn state_current_internal(
        data: FileDownloadData<'_, Id>,
        dest: &Path,
    ) -> Result<FileDownloadState, FileDownloadError> {
        #[cfg(not(target_arch = "wasm32"))]
        let file_exists = dest.exists();
        #[cfg(target_arch = "wasm32")]
        let file_exists = data.storage().get_item_opt(dest)?.is_some();
        if !file_exists {
            return Ok(FileDownloadState::new(
                FileDownloadStateLogical::None {
                    path: Some(dest.to_path_buf()),
                },
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
            .map(|state_working| state_working.0.physical.clone())
            .or_else(|| {
                data.state_prev()
                    .get()
                    .map(|state_prev| state_prev.0.physical.clone())
            })
            .unwrap_or(if let FileDownloadStateLogical::None { .. } = &file_state {
                FetchedOpt::Tbd
            } else {
                FetchedOpt::None
            });

        Ok(FileDownloadState::new(file_state, e_tag))
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn read_file_contents(
        dest: &std::path::Path,
    ) -> Result<FileDownloadStateLogical, FileDownloadError> {
        let mut file = File::open(dest)
            .await
            .map_err(FileDownloadError::DestFileOpen)?;
        let metadata = file
            .metadata()
            .await
            .map_err(FileDownloadError::DestMetadataRead)?;
        let file_state = if metadata.len() > crate::IN_MEMORY_CONTENTS_MAX {
            FileDownloadStateLogical::Unknown {
                path: dest.to_path_buf(),
            }
        } else {
            let mut buffer = String::new();

            file.read_to_string(&mut buffer)
                .await
                .map_err(FileDownloadError::DestFileRead)?;
            FileDownloadStateLogical::StringContents {
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
    ) -> Result<FileDownloadStateLogical, FileDownloadError> {
        let file_state = storage
            .get_item_opt(dest)?
            .map(|contents| {
                contents
                    .bytes()
                    .len()
                    .try_into()
                    .map(|byte_count: u64| {
                        if byte_count > crate::IN_MEMORY_CONTENTS_MAX {
                            FileDownloadStateLogical::Unknown {
                                path: dest.to_path_buf(),
                            }
                        } else {
                            FileDownloadStateLogical::StringContents {
                                path: dest.to_path_buf(),
                                contents: contents.clone(),
                            }
                        }
                    })
                    .unwrap_or_else(|_| FileDownloadStateLogical::StringContents {
                        path: dest.to_path_buf(),
                        contents: contents.clone(),
                    })
            })
            .unwrap_or(FileDownloadStateLogical::None {
                path: Some(dest.to_path_buf()),
            });

        Ok(file_state)
    }
}
