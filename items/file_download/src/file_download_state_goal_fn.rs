use std::{marker::PhantomData, path::Path};

use peace::{
    cfg::{state::FetchedOpt, FnCtx},
    params::Params,
};
use reqwest::{header::ETAG, Url};

use crate::{
    ETag, FileDownloadData, FileDownloadError, FileDownloadParams, FileDownloadState,
    FileDownloadStateLogical,
};

/// Reads the goal state of the file to download.
#[derive(Debug)]
pub struct FileDownloadStateGoalFn<Id>(PhantomData<Id>);

impl<Id> FileDownloadStateGoalFn<Id>
where
    Id: Send + Sync + 'static,
{
    pub async fn try_state_goal(
        _fn_ctx: FnCtx<'_>,
        params_partial: &<FileDownloadParams<Id> as Params>::Partial,
        data: FileDownloadData<'_, Id>,
    ) -> Result<Option<FileDownloadState>, FileDownloadError> {
        if let Some((src, dest)) = params_partial.src().zip(params_partial.dest()) {
            Self::file_state_goal(&data, src, dest).await.map(Some)
        } else {
            Ok(None)
        }
    }

    pub async fn state_goal(
        _fn_ctx: FnCtx<'_>,
        params: &FileDownloadParams<Id>,
        data: FileDownloadData<'_, Id>,
    ) -> Result<FileDownloadState, FileDownloadError> {
        let file_state_goal = Self::file_state_goal(&data, params.src(), params.dest()).await?;

        Ok(file_state_goal)
    }

    async fn file_state_goal(
        data: &FileDownloadData<'_, Id>,
        src_url: &Url,
        dest: &Path,
    ) -> Result<FileDownloadState, FileDownloadError> {
        let client = data.client();
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

                    FileDownloadStateLogical::StringContents {
                        path: dest.to_path_buf(),
                        contents: remote_contents,
                    }
                } else {
                    // Stream it later.
                    FileDownloadStateLogical::Length {
                        path: dest.to_path_buf(),
                        byte_count: remote_file_length,
                    }
                }
            } else {
                FileDownloadStateLogical::Unknown {
                    path: dest.to_path_buf(),
                }
            };

            Ok(FileDownloadState::new(file_download_state, e_tag))
        } else {
            Err(FileDownloadError::SrcFileUndetermined { status_code })
        }
    }
}
