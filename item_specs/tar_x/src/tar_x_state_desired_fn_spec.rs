use std::{io::Read, marker::PhantomData, path::Path};

use peace::{cfg::OpCtx, rt_model::Storage};
use tar::Archive;

use crate::{FileMetadata, FileMetadatas, TarXData, TarXError};

/// Reads the desired state of the tar to extract.
#[derive(Debug)]
pub struct TarXStateDesiredFnSpec<Id>(PhantomData<Id>);

impl<Id> TarXStateDesiredFnSpec<Id>
where
    Id: Send + Sync,
{
    pub async fn try_state_desired(
        op_ctx: OpCtx<'_>,
        tar_x_data: TarXData<'_, Id>,
    ) -> Result<Option<FileMetadatas>, TarXError> {
        #[cfg(not(target_arch = "wasm32"))]
        let tar_file_exists = tar_x_data.tar_x_params().tar_path().exists();
        #[cfg(target_arch = "wasm32")]
        let tar_file_exists = {
            let storage = tar_x_data.storage();
            let tar_path = tar_x_data.tar_x_params().tar_path();
            storage.contains_item(tar_path)?
        };

        if tar_file_exists {
            Self::state_desired(op_ctx, tar_x_data).await.map(Some)
        } else {
            Ok(None)
        }
    }

    pub async fn state_desired(
        _op_ctx: OpCtx<'_>,
        tar_x_data: TarXData<'_, Id>,
    ) -> Result<FileMetadatas, TarXError> {
        let tar_x_params = tar_x_data.tar_x_params();
        let storage = tar_x_data.storage();
        let tar_path = tar_x_params.tar_path();

        #[cfg(not(target_arch = "wasm32"))]
        let tar_file_exists = tar_x_data.tar_x_params().tar_path().exists();
        #[cfg(target_arch = "wasm32")]
        let tar_file_exists = storage.contains_item(tar_path)?;

        if tar_file_exists {
            #[cfg(not(target_arch = "wasm32"))]
            let files_in_tar = Self::files_in_tar(storage, tar_path).await?;
            #[cfg(target_arch = "wasm32")]
            let files_in_tar = Self::files_in_tar(storage, tar_path)?;

            Ok(FileMetadatas::from(files_in_tar))
        } else {
            let tar_path = tar_path.to_path_buf();
            Err(TarXError::TarFileNotExists { tar_path })
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn files_in_tar(
        storage: &Storage,
        tar_path: &Path,
    ) -> Result<Vec<FileMetadata>, TarXError> {
        let file_metadatas = storage
            .read_with_sync_api(
                "TarXStateDesiredFnSpec::files_in_tar".to_string(),
                tar_path,
                |sync_io_bridge| Self::tar_file_metadata(tar_path, Archive::new(sync_io_bridge)),
            )
            .await?;

        Ok(file_metadatas)
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

                // Ignore directories in tracked `FileMetadata`s, because:
                //
                // * mtime of tar entries is the mtime it was created.
                // * mtime of directories on the file system is always the time it is unpacked,
                //   even if the unpack is told to `preserve_mtime`.
                if entry.header().entry_type().is_dir() {
                    return Ok(files_in_tar);
                }

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
