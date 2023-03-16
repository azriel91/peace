use std::marker::PhantomData;

use peace::cfg::{async_trait, TryFnSpec};

use crate::item_specs::peace_aws_s3_object::{S3ObjectData, S3ObjectError, S3ObjectState};

/// Reads the desired state of the S3 object state.
#[derive(Debug)]
pub struct S3ObjectStateDesiredFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> TryFnSpec for S3ObjectStateDesiredFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = S3ObjectData<'op, Id>;
    type Error = S3ObjectError;
    type Output = S3ObjectState;

    async fn try_exec(
        s3_object_data: S3ObjectData<'_, Id>,
    ) -> Result<Option<Self::Output>, S3ObjectError> {
        Self::exec(s3_object_data).await.map(Some)
    }

    async fn exec(s3_object_data: S3ObjectData<'_, Id>) -> Result<Self::Output, S3ObjectError> {
        let params = s3_object_data.params();
        let bucket_name = params.bucket_name().to_string();
        let object_key = params.object_key().to_string();
        let file_path = params.file_path();

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

        dbg!(content_md5_bytes);

        let content_md5_hexstr = content_md5_bytes
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>();

        Ok(S3ObjectState::Some {
            bucket_name,
            object_key,
            content_md5_hexstr,
        })
    }
}
