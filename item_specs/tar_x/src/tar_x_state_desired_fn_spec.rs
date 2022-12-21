use std::{io::Read, marker::PhantomData, path::Path};

use peace::{
    cfg::{async_trait, FnSpec},
    rt_model::Storage,
};
use tar::Archive;

use crate::{FileMetadata, FileMetadatas, TarXData, TarXError};

/// Status desired `FnSpec` for the tar to extract.
#[derive(Debug)]
pub struct TarXStateDesiredFnSpec<Id>(PhantomData<Id>);

impl<Id> TarXStateDesiredFnSpec<Id> {
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn files_in_tar(
        storage: &Storage,
        tar_path: &Path,
    ) -> Result<Vec<FileMetadata>, TarXError> {
        storage
            .read_with_sync_api(
                "TarXStateDesiredFnSpec::files_in_tar".to_string(),
                tar_path,
                |sync_io_bridge| Self::tar_file_metadata(tar_path, Archive::new(sync_io_bridge)),
            )
            .await
    }

    #[cfg(target_arch = "wasm32")]
    pub fn files_in_tar(
        storage: &Storage,
        tar_path: &Path,
    ) -> Result<Vec<FileMetadata>, TarXError> {
        use std::io::Cursor;

        let bytes = storage.get_item_b64(tar_path)?;
        Self::tar_file_metadata(tar_path, Archive::new(Cursor::new(bytes)))
    }

    fn tar_file_metadata<R>(
        tar_path: &Path,
        mut archive: Archive<R>,
    ) -> Result<Vec<FileMetadata>, TarXError>
    where
        R: Read,
    {
        archive
            .entries()
            .map_err(|error| {
                let tar_path = tar_path.to_path_buf();
                TarXError::TarEntryRead { tar_path, error }
            })?
            .try_fold(Vec::new(), |mut files_in_tar, entry| {
                let entry = entry.map_err(|error| {
                    let tar_path = tar_path.to_path_buf();
                    TarXError::TarEntryRead { tar_path, error }
                })?;
                let entry_path = entry.path().map_err(|error| {
                    let tar_path = tar_path.to_path_buf();
                    TarXError::TarEntryPathRead { tar_path, error }
                })?;
                let modified_time = entry.header().mtime().map_err(|error| {
                    let tar_path = tar_path.to_path_buf();
                    let entry_path = entry_path.to_path_buf();
                    TarXError::TarEntryMTimeRead {
                        tar_path,
                        entry_path,
                        error,
                    }
                })?;

                let file_metadata = FileMetadata::new(entry_path.to_path_buf(), modified_time);
                files_in_tar.push(file_metadata);

                Ok(files_in_tar)
            })
    }
}

#[async_trait(?Send)]
impl<Id> FnSpec for TarXStateDesiredFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = TarXData<'op, Id>;
    type Error = TarXError;
    type Output = FileMetadatas;

    async fn exec(tar_x_data: TarXData<'_, Id>) -> Result<Self::Output, TarXError> {
        let tar_x_params = tar_x_data.tar_x_params();
        let storage = tar_x_data.storage();
        let tar_path = tar_x_params.tar_path();

        #[cfg(not(target_arch = "wasm32"))]
        let files_in_tar = Self::files_in_tar(storage, tar_path).await?;
        #[cfg(target_arch = "wasm32")]
        let files_in_tar = Self::files_in_tar(storage, tar_path)?;

        Ok(FileMetadatas::new(files_in_tar))
    }
}
