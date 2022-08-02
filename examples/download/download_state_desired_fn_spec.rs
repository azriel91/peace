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
        let src_url = download_params.src();
        let response = client
            .get(src_url.clone())
            .send()
            .await
            .map_err(DownloadError::SrcGet)?;

        let status_code = response.status();
        if status_code.is_success() {
            if let Some(remote_file_length) = response.content_length() {
                if remote_file_length <= crate::IN_MEMORY_CONTENTS_MAX {
                    // Download it now.
                    let remote_contents =
                        response.text().await.map_err(DownloadError::SrcFileRead)?;
                    Ok(FileState::StringContents(remote_contents))
                } else {
                    // Stream it later.
                    Ok(FileState::Length(remote_file_length))
                }
            } else {
                Ok(FileState::Unknown)
            }
        } else {
            Err(DownloadError::SrcFileUndetermined { status_code })
        }
    }
}

#[async_trait]
#[nougat::gat]
impl FnSpec for DownloadStateDesiredFnSpec {
    type Data<'op> = DownloadParams<'op>
        where Self: 'op;
    type Error = DownloadError;
    type Output = Option<FileState>;

    async fn exec(download_params: DownloadParams<'_>) -> Result<Self::Output, DownloadError> {
        let file_state_desired = Self::file_state_desired(&download_params).await?;

        Ok(Some(file_state_desired))
    }
}
