use peace::cfg::{async_trait, OpCheckStatus, OpSpec, ProgressLimit};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{DownloadError, DownloadParams, FileState};

/// Status OpSpec for the file to download.
#[derive(Debug, Default)]
pub struct DownloadStatusOpSpec;

#[async_trait]
impl<'op> OpSpec<'op> for DownloadStatusOpSpec {
    type Data = DownloadParams<'op>;
    type Error = DownloadError;
    type Output = Option<FileState>;
    type State = ();

    async fn check(_: DownloadParams<'op>, _: &()) -> Result<OpCheckStatus, DownloadError> {
        Ok(OpCheckStatus::ExecRequired {
            // Need to make one request.
            progress_limit: ProgressLimit::Steps(1),
        })
    }

    async fn exec(
        download_params: DownloadParams<'op>,
    ) -> Result<Option<FileState>, DownloadError> {
        // Destination file doesn't exist.
        let dest = download_params.dest().ok_or(DownloadError::DestFileInit)?;
        if !dest.exists() {
            return Ok(None);
        }

        // Check file length
        let mut file = File::open(dest)
            .await
            .map_err(DownloadError::DestFileOpen)?;
        let metadata = file
            .metadata()
            .await
            .map_err(DownloadError::DestMetadataRead)?;

        let state = if metadata.len() > crate::IN_MEMORY_CONTENTS_MAX {
            Some(FileState::Length(metadata.len()))
        } else {
            let mut buffer = String::new();

            file.read_to_string(&mut buffer)
                .await
                .map_err(DownloadError::DestFileRead)?;
            Some(FileState::StringContents(buffer))
        };

        Ok(state)
    }
}
