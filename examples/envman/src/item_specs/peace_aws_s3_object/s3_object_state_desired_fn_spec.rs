use std::marker::PhantomData;

use peace::cfg::{state::Generated, OpCtx};

use crate::item_specs::peace_aws_s3_object::{S3ObjectData, S3ObjectError, S3ObjectState};

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressMsgUpdate;

/// Reads the desired state of the S3 object state.
#[derive(Debug)]
pub struct S3ObjectStateDesiredFnSpec<Id>(PhantomData<Id>);

impl<Id> S3ObjectStateDesiredFnSpec<Id>
where
    Id: Send + Sync,
{
    pub async fn try_state_desired(
        op_ctx: OpCtx<'_>,
        s3_object_data: S3ObjectData<'_, Id>,
    ) -> Result<Option<S3ObjectState>, S3ObjectError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let params = s3_object_data.params();
            let file_path = params.file_path();
            let bucket_name = params.bucket_name();
            let object_key = params.object_key();
            if !tokio::fs::try_exists(file_path).await.map_err(|error| {
                S3ObjectError::ObjectFileExists {
                    file_path: file_path.to_path_buf(),
                    bucket_name: bucket_name.to_string(),
                    object_key: object_key.to_string(),
                    error,
                }
            })? {
                return Ok(None);
            }
        }
        Self::state_desired(op_ctx, s3_object_data).await.map(Some)
    }

    pub async fn state_desired(
        op_ctx: OpCtx<'_>,
        data: S3ObjectData<'_, Id>,
    ) -> Result<S3ObjectState, S3ObjectError> {
        let params = data.params();
        let file_path = params.file_path();
        let bucket_name = params.bucket_name().to_string();
        let object_key = params.object_key().to_string();

        #[cfg(not(feature = "output_progress"))]
        let _op_ctx = op_ctx;
        #[cfg(feature = "output_progress")]
        let progress_sender = &op_ctx.progress_sender;
        #[cfg(feature = "output_progress")]
        progress_sender.tick(ProgressMsgUpdate::Set(String::from("computing MD5 sum")));

        #[cfg(not(target_arch = "wasm32"))]
        let content_md5_bytes = {
            use tokio::{fs::File, io::AsyncReadExt};

            let mut md5_ctx = md5_rs::Context::new();
            let mut file =
                File::open(file_path)
                    .await
                    .map_err(|error| S3ObjectError::ObjectFileOpen {
                        file_path: file_path.to_path_buf(),
                        bucket_name: bucket_name.clone(),
                        object_key: object_key.clone(),
                        error,
                    })?;

            let mut bytes_buffer = [0u8; 1024];
            loop {
                match file.read(&mut bytes_buffer).await.map_err(|error| {
                    S3ObjectError::ObjectFileRead {
                        file_path: file_path.to_path_buf(),
                        bucket_name: bucket_name.clone(),
                        object_key: object_key.clone(),
                        error,
                    }
                })? {
                    0 => break md5_ctx.finish(),
                    n => {
                        md5_ctx.read(&bytes_buffer[..n]);
                    }
                }
            }
        };
        #[cfg(feature = "output_progress")]
        progress_sender.tick(ProgressMsgUpdate::Set(String::from("MD5 sum computed")));

        let content_md5_hexstr = content_md5_bytes
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>();

        Ok(S3ObjectState::Some {
            bucket_name,
            object_key,
            content_md5_hexstr: Some(content_md5_hexstr),
            e_tag: Generated::Tbd,
        })
    }
}
