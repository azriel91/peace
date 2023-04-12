use std::marker::PhantomData;

use peace::cfg::{state::FetchedOpt, FnCtx, State};
use reqwest::header::ETAG;

use crate::{ETag, FileDownloadData, FileDownloadError, FileDownloadState};

/// Reads the desired state of the file to download.
#[derive(Debug)]
pub struct FileDownloadStateDesiredFn<Id>(PhantomData<Id>);

impl<Id> FileDownloadStateDesiredFn<Id>
where
    Id: Send + Sync + 'static,
{
    pub async fn try_state_desired(
        fn_ctx: FnCtx<'_>,
        data: FileDownloadData<'_, Id>,
    ) -> Result<Option<State<FileDownloadState, FetchedOpt<ETag>>>, FileDownloadError> {
        Self::state_desired(fn_ctx, data).await.map(Some)
    }

    pub async fn state_desired(
        fn_ctx: FnCtx<'_>,
        data: FileDownloadData<'_, Id>,
    ) -> Result<State<FileDownloadState, FetchedOpt<ETag>>, FileDownloadError> {
        let file_state_desired = Self::file_state_desired(fn_ctx, &data).await?;

        Ok(file_state_desired)
    }

    async fn file_state_desired(
        _fn_ctx: FnCtx<'_>,
        data: &FileDownloadData<'_, Id>,
    ) -> Result<State<FileDownloadState, FetchedOpt<ETag>>, FileDownloadError> {
        let client = data.client();
        let file_download_params = data.file_download_params();
        let dest = file_download_params.dest();
        let src_url = file_download_params.src();
        let response = client
            .get(src_url.clone())
            .send()
            .await
            .map_err(|error| FileDownloadError::src_get(src_url.clone(), error))?;

        let status_code = response.status();
        if status_code.is_success() {
            let content_length = response.content_length();
            let e_tag = response
                .headers()
                .get(ETAG)
                .and_then(|header| header.to_str().ok())
                .map(|header| ETag::new(header.to_string()))
                .map(FetchedOpt::Value)
                .unwrap_or(FetchedOpt::None);

            let file_download_state = if let Some(remote_file_length) = content_length {
                if remote_file_length <= crate::IN_MEMORY_CONTENTS_MAX {
                    // Download it now.
                    let remote_contents = async move {
                        let response_text = response.text();
                        response_text.await.map_err(FileDownloadError::SrcFileRead)
                    }
                    .await?;

                    FileDownloadState::StringContents {
                        path: dest.to_path_buf(),
                        contents: remote_contents,
                    }
                } else {
                    // Stream it later.
                    FileDownloadState::Length {
                        path: dest.to_path_buf(),
                        byte_count: remote_file_length,
                    }
                }
            } else {
                FileDownloadState::Unknown {
                    path: dest.to_path_buf(),
                }
            };

            Ok(State::new(file_download_state, e_tag))
        } else {
            Err(FileDownloadError::SrcFileUndetermined { status_code })
        }
    }
}
