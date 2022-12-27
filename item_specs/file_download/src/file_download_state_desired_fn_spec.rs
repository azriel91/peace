use std::marker::PhantomData;

use peace::cfg::{async_trait, TryFnSpec};

use crate::{FileDownloadData, FileDownloadError, FileDownloadState};

/// Reads the desired state of the file to download.
#[derive(Debug)]
pub struct FileDownloadStateDesiredFnSpec<Id>(PhantomData<Id>);

impl<Id> FileDownloadStateDesiredFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    async fn file_state_desired(
        file_download_data: &FileDownloadData<'_, Id>,
    ) -> Result<FileDownloadState, FileDownloadError> {
        let client = file_download_data.client();
        let file_download_params = file_download_data.file_download_params();
        let dest = file_download_params.dest();
        let src_url = file_download_params.src();
        let response = client.get(src_url.clone()).send().await.map_err(|error| {
            #[cfg(not(target_arch = "wasm32"))]
            let (Ok(file_download_error) | Err(file_download_error)) =
                FileDownloadError::src_get(src_url.clone(), dest, error);
            #[cfg(target_arch = "wasm32")]
            let file_download_error = FileDownloadError::src_get(src_url.clone(), error);

            file_download_error
        })?;

        let status_code = response.status();
        if status_code.is_success() {
            let content_length = response.content_length();
            if let Some(remote_file_length) = content_length {
                if remote_file_length <= crate::IN_MEMORY_CONTENTS_MAX {
                    // Download it now.
                    let remote_contents = async move {
                        let response_text = response.text();
                        response_text.await.map_err(FileDownloadError::SrcFileRead)
                    }
                    .await?;
                    Ok(FileDownloadState::StringContents {
                        path: dest.to_path_buf(),
                        contents: remote_contents,
                    })
                } else {
                    // Stream it later.
                    Ok(FileDownloadState::Length {
                        path: dest.to_path_buf(),
                        byte_count: remote_file_length,
                    })
                }
            } else {
                Ok(FileDownloadState::Unknown {
                    path: dest.to_path_buf(),
                })
            }
        } else {
            Err(FileDownloadError::SrcFileUndetermined { status_code })
        }
    }
}

#[async_trait(?Send)]
impl<Id> TryFnSpec for FileDownloadStateDesiredFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = FileDownloadData<'op, Id>;
    type Error = FileDownloadError;
    type Output = Option<FileDownloadState>;

    async fn exec(
        file_download_data: FileDownloadData<'_, Id>,
    ) -> Result<Self::Output, FileDownloadError> {
        let file_state_desired = Self::file_state_desired(&file_download_data).await?;

        Ok(Some(file_state_desired))
    }
}
