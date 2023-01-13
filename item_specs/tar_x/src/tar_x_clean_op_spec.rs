use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, CleanOpSpec, OpCheckStatus};

use crate::{FileMetadatas, TarXData, TarXError};

/// `CleanOpSpec` for the tar to extract.
#[derive(Debug, Default)]
pub struct TarXCleanOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> CleanOpSpec for TarXCleanOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = TarXData<'op, Id>;
    type Error = TarXError;
    type State = FileMetadatas;

    async fn check(
        _tar_x_data: TarXData<'_, Id>,
        file_metadatas: &FileMetadatas,
    ) -> Result<OpCheckStatus, TarXError> {
        let op_check_status = if file_metadatas.is_empty() {
            OpCheckStatus::ExecNotRequired
        } else {
            #[cfg(not(feature = "output_progress"))]
            {
                OpCheckStatus::ExecRequired
            }
            #[cfg(feature = "output_progress")]
            {
                let progress_limit = ProgressLimit::Steps(file_metadatas.len().try_into().unwrap());
                OpCheckStatus::ExecRequired { progress_limit }
            }
        };

        Ok(op_check_status)
    }

    async fn exec_dry(
        _tar_x_data: TarXData<'_, Id>,
        _state_current: &FileMetadatas,
    ) -> Result<(), TarXError> {
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn exec(
        tar_x_data: TarXData<'_, Id>,
        file_metadatas: &FileMetadatas,
    ) -> Result<(), TarXError> {
        use futures::stream::{StreamExt, TryStreamExt};

        let params = tar_x_data.tar_x_params();
        let dest = params.dest();

        if dest.exists() {
            // Remove files in the destination directory that are tracked by the state.
            //
            // We could technically remove the directory, but this approach is closer to the
            // implementation where we would support not removing files not tracked by the
            // tar.
            futures::stream::iter(file_metadatas.iter())
                .map(|file_metadata| Result::<_, TarXError>::Ok(file_metadata.path()))
                .try_for_each_concurrent(None, |entry_path| async move {
                    tokio::fs::remove_file(&dest.join(entry_path))
                        .await
                        .map_err(|error| TarXError::TarDestFileRemove {
                            dest: dest.to_path_buf(),
                            entry_path: entry_path.to_path_buf(),
                            error,
                        })
                })
                .await?;
        }

        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    async fn exec(
        _tar_x_data: TarXData<'_, Id>,
        _state_current: &FileMetadatas,
    ) -> Result<(), TarXError> {
        todo!()
    }
}
