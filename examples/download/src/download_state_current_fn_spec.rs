#[nougat::gat(Data)]
use peace::cfg::FnSpec;
use peace::cfg::{async_trait, nougat, state::Nothing, State};
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
    async fn read_file_contents(dest: &std::path::Path) -> Result<FileState, DownloadError> {
        let mut file = File::open(dest)
            .await
            .map_err(DownloadError::DestFileOpen)?;
        let metadata = file
            .metadata()
            .await
            .map_err(DownloadError::DestMetadataRead)?;
        let file_state = if metadata.len() > crate::IN_MEMORY_CONTENTS_MAX {
            FileState::Length {
                path: dest.to_path_buf(),
                byte_count: metadata.len(),
            }
        } else {
            let mut buffer = String::new();

            file.read_to_string(&mut buffer)
                .await
                .map_err(DownloadError::DestFileRead)?;
            FileState::StringContents {
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
    ) -> Result<FileState, DownloadError> {
        let file_state = storage
            .get_item_opt(dest)?
            .map(|contents| {
                contents
                    .bytes()
                    .len()
                    .try_into()
                    .map(|byte_count| {
                        if byte_count > crate::IN_MEMORY_CONTENTS_MAX {
                            FileState::Length {
                                path: dest.to_path_buf(),
                                byte_count,
                            }
                        } else {
                            FileState::StringContents {
                                path: dest.to_path_buf(),
                                contents: contents.clone(),
                            }
                        }
                    })
                    .unwrap_or_else(|_| FileState::StringContents {
                        path: dest.to_path_buf(),
                        contents: contents.clone(),
                    })
            })
            .unwrap_or(FileState::None {
                path: dest.to_path_buf(),
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
    type Output = State<FileState, Nothing>;

    async fn exec(download_params: DownloadParams<'_>) -> Result<Self::Output, DownloadError> {
        let dest = download_params.download_profile_init().dest();

        #[cfg(not(target_arch = "wasm32"))]
        let file_exists = dest.exists();
        #[cfg(target_arch = "wasm32")]
        let file_exists = download_params.storage().get_item_opt(dest)?.is_some();
        if !file_exists {
            let path = dest.to_path_buf();
            return Ok(State::new(FileState::None { path }, Nothing));
        }

        // Check file length
        #[cfg(not(target_arch = "wasm32"))]
        let file_state = Self::read_file_contents(dest).await?;

        #[cfg(target_arch = "wasm32")]
        let file_state = Self::read_file_contents(dest, download_params.storage()).await?;

        Ok(State::new(file_state, Nothing))
    }
}
