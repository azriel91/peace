use std::path::PathBuf;

#[nougat::gat(Data)]
use peace::cfg::FnSpec;
use peace::cfg::{async_trait, nougat, State};
#[cfg(not(target_arch = "wasm32"))]
use tokio::{fs::File, io::AsyncReadExt};

#[cfg(target_arch = "wasm32")]
use peace::rt_model::Storage;

use crate::{DownloadError, DownloadParams, FileState};

/// Status `FnSpec` for the file to download.
#[derive(Debug)]
pub struct DownloadStateCurrentFnSpec;

impl DownloadStateCurrentFnSpec {
    #[cfg(not(target_arch = "wasm32"))]
    async fn read_file_contents(
        dest: &std::path::Path,
    ) -> Result<Option<FileState>, DownloadError> {
        let mut file = File::open(dest)
            .await
            .map_err(DownloadError::DestFileOpen)?;
        let metadata = file
            .metadata()
            .await
            .map_err(DownloadError::DestMetadataRead)?;
        let file_state = if metadata.len() > crate::IN_MEMORY_CONTENTS_MAX {
            Some(FileState::Length(metadata.len()))
        } else {
            let mut buffer = String::new();

            file.read_to_string(&mut buffer)
                .await
                .map_err(DownloadError::DestFileRead)?;
            Some(FileState::StringContents(buffer))
        };
        Ok(file_state)
    }

    #[cfg(target_arch = "wasm32")]
    async fn read_file_contents(
        dest: &std::path::Path,
        storage: &Storage,
    ) -> Result<Option<FileState>, DownloadError> {
        let file_state = storage.get_item_opt(dest)?.map(|contents| {
            contents
                .bytes()
                .len()
                .try_into()
                .map(|byte_len| {
                    if byte_len > crate::IN_MEMORY_CONTENTS_MAX {
                        FileState::Length(byte_len)
                    } else {
                        FileState::StringContents(contents.clone())
                    }
                })
                .unwrap_or_else(|_| FileState::StringContents(contents.clone()))
        });

        Ok(file_state)
    }
}

#[async_trait(?Send)]
#[nougat::gat]
impl FnSpec for DownloadStateCurrentFnSpec {
    type Data<'op> = DownloadParams<'op>
        where Self: 'op;
    type Error = DownloadError;
    type Output = State<Option<FileState>, PathBuf>;

    async fn exec(download_params: DownloadParams<'_>) -> Result<Self::Output, DownloadError> {
        let dest = download_params.download_profile_init().dest();

        #[cfg(not(target_arch = "wasm32"))]
        let file_exists = dest.exists();
        #[cfg(target_arch = "wasm32")]
        let file_exists = download_params.storage().get_item_opt(dest)?.is_some();
        if !file_exists {
            return Ok(State::new(None, dest.to_path_buf()));
        }

        // Check file length
        #[cfg(not(target_arch = "wasm32"))]
        let file_state = Self::read_file_contents(dest).await?;

        #[cfg(target_arch = "wasm32")]
        let file_state = Self::read_file_contents(dest, download_params.storage()).await?;

        Ok(State::new(file_state, dest.to_path_buf()))
    }
}
