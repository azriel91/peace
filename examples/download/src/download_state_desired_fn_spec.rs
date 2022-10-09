#[nougat::gat(Data)]
use peace::cfg::FnSpec;
use peace::cfg::{async_trait, nougat};

use crate::{DownloadError, DownloadParams, FileState};

/// Status desired `FnSpec` for the file to download.
#[derive(Debug)]
pub struct DownloadStateDesiredFnSpec;

impl DownloadStateDesiredFnSpec {
    async fn file_state_desired(
        download_params: &DownloadParams<'_>,
    ) -> Result<FileState, DownloadError> {
        let client = download_params.client();
        let dest = download_params.download_profile_init().dest();
        let src_url = download_params.download_profile_init().src();
        let response = client
            .get(src_url.clone())
            .send()
            .await
            .map_err(DownloadError::SrcGet)?;

        let status_code = response.status();
        if status_code.is_success() {
            let content_length = response.content_length();
            if let Some(remote_file_length) = content_length {
                if remote_file_length <= crate::IN_MEMORY_CONTENTS_MAX {
                    // Download it now.
                    let remote_contents = async move {
                        let response_text = response.text();
                        response_text.await.map_err(DownloadError::SrcFileRead)
                    }
                    .await?;
                    Ok(FileState::StringContents {
                        path: dest.to_path_buf(),
                        contents: remote_contents,
                    })
                } else {
                    // Stream it later.
                    Ok(FileState::Length {
                        path: dest.to_path_buf(),
                        byte_count: remote_file_length,
                    })
                }
            } else {
                Ok(FileState::Unknown {
                    path: dest.to_path_buf(),
                })
            }
        } else {
            Err(DownloadError::SrcFileUndetermined { status_code })
        }
    }
}

#[async_trait(?Send)]
#[nougat::gat]
impl FnSpec for DownloadStateDesiredFnSpec {
    type Data<'op> = DownloadParams<'op>
        where Self: 'op;
    type Error = DownloadError;
    type Output = FileState;

    async fn exec(download_params: DownloadParams<'_>) -> Result<Self::Output, DownloadError> {
        let file_state_desired = Self::file_state_desired(&download_params).await?;

        Ok(file_state_desired)
    }
}
