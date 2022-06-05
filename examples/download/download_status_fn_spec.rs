use peace::cfg::{async_trait, FnSpec};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{DownloadError, DownloadParams, FileState};

/// Status `FnSpec` for the file to download.
#[derive(Debug, Default)]
pub struct DownloadStatusFnSpec;

#[async_trait]
impl<'op> FnSpec<'op> for DownloadStatusFnSpec {
    type Data = DownloadParams<'op>;
    type Error = DownloadError;
    type Output = Option<FileState>;

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
