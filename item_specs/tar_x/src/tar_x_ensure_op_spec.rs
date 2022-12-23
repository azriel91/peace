use std::marker::PhantomData;

use peace::cfg::state::Nothing;

use peace::cfg::{async_trait, EnsureOpSpec, OpCheckStatus, ProgressLimit, State};

use crate::{FileMetadatas, TarXData, TarXError, TarXStateDiff};

/// Ensure OpSpec for the tar to extract.
#[derive(Debug)]
pub struct TarXEnsureOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> EnsureOpSpec for TarXEnsureOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = TarXData<'op, Id>;
    type Error = TarXError;
    type StateDiff = TarXStateDiff;
    type StateLogical = FileMetadatas;
    type StatePhysical = Nothing;

    async fn check(
        _tar_x_data: TarXData<'_, Id>,
        _state_current: &State<FileMetadatas, Nothing>,
        state_desired: &FileMetadatas,
        diff: &TarXStateDiff,
    ) -> Result<OpCheckStatus, TarXError> {
        let op_check_status = match diff {
            TarXStateDiff::ExtractionInSync => OpCheckStatus::ExecNotRequired,
            TarXStateDiff::ExtractionOutOfSync {
                added: _,
                modified: _,
                removed: _,
            } => {
                let progress_limit = ProgressLimit::Steps(state_desired.len().try_into().unwrap());
                OpCheckStatus::ExecRequired { progress_limit }
            }
        };

        Ok(op_check_status)
    }

    async fn exec_dry(
        _tar_x_data: TarXData<'_, Id>,
        _state_current: &State<FileMetadatas, Nothing>,
        _state_desired: &FileMetadatas,
        _diff: &TarXStateDiff,
    ) -> Result<Nothing, TarXError> {
        Ok(Nothing)
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn exec(
        tar_x_data: TarXData<'_, Id>,
        _state_current: &State<FileMetadatas, Nothing>,
        _state_desired: &FileMetadatas,
        diff: &TarXStateDiff,
    ) -> Result<Nothing, TarXError> {
        use futures::stream::{StreamExt, TryStreamExt};

        let storage = tar_x_data.storage();
        let params = tar_x_data.tar_x_params();
        let tar_path = params.tar_path();
        let dest = params.dest();

        tokio::fs::create_dir_all(dest)
            .await
            .map_err(|error| TarXError::TarDestDirCreate {
                dest: dest.to_path_buf(),
                error,
            })?;

        // TODO: Optimize by unpacking only the entries that changed.
        // Probably store entries in `IndexMap`s, then look them up to determine if they
        // need to be unpacked.
        storage
            .read_with_sync_api(
                "TarXEnsureOpSpec::exec".to_string(),
                tar_path,
                |sync_io_bridge| {
                    tar::Archive::new(sync_io_bridge)
                        .unpack(dest)
                        .map_err(|error| TarXError::TarUnpack {
                            tar_path: tar_path.to_path_buf(),
                            dest: dest.to_path_buf(),
                            error,
                        })?;
                    Result::<_, TarXError>::Ok(())
                },
            )
            .await?;

        if let TarXStateDiff::ExtractionOutOfSync {
            added: _,
            modified: _,
            removed,
        } = diff
        {
            // Remove files that are not in the tar, but are in the destination directory.
            futures::stream::iter(removed.iter())
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

        Ok(Nothing)
    }

    #[cfg(target_arch = "wasm32")]
    async fn exec(
        _tar_x_data: TarXData<'_, Id>,
        _state_current: &State<FileMetadatas, Nothing>,
        _state_desired: &FileMetadatas,
        _diff: &TarXStateDiff,
    ) -> Result<Nothing, TarXError> {
        todo!()
    }
}
