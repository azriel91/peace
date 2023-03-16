use std::marker::PhantomData;

use aws_sdk_iam::types::SdkError;
use aws_sdk_s3::error::HeadObjectErrorKind;
use base64::Engine;
use peace::cfg::{async_trait, TryFnSpec};

use crate::item_specs::peace_aws_s3_object::{S3ObjectData, S3ObjectError, S3ObjectState};

/// Reads the current state of the S3 object state.
#[derive(Debug)]
pub struct S3ObjectStateCurrentFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> TryFnSpec for S3ObjectStateCurrentFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = S3ObjectData<'op, Id>;
    type Error = S3ObjectError;
    type Output = S3ObjectState;

    async fn try_exec(data: S3ObjectData<'_, Id>) -> Result<Option<Self::Output>, S3ObjectError> {
        Self::exec(data).await.map(Some)
    }

    async fn exec(data: S3ObjectData<'_, Id>) -> Result<Self::Output, S3ObjectError> {
        let client = data.client();
        let bucket_name = data.params().bucket_name();
        let object_key = data.params().object_key();

        let head_object_result = client
            .head_object()
            .bucket(bucket_name)
            .key(object_key)
            .send()
            .await;
        let content_md5_b64 = match head_object_result {
            Ok(head_object_output) => {
                let content_md5_b64 = head_object_output
                    .e_tag()
                    .expect("Expected S3 object e_tag to be Some when head_object is successful.")
                    .to_string();

                Some(content_md5_b64)
            }
            Err(error) => match &error {
                SdkError::ServiceError(service_error) => {
                    dbg!(&service_error);

                    // If your user does not have permissions, AWS SDK Rust does not return an
                    // access denied error. It just returns "Error"
                    // with no other information.
                    // https://github.com/awslabs/aws-sdk-rust/issues/227

                    match service_error.err().kind {
                        HeadObjectErrorKind::NotFound(_) => None,
                        _ => {
                            return Err(S3ObjectError::S3ObjectGetError {
                                object_key: object_key.to_string(),
                                error,
                            });
                        }
                    }
                }
                _ => {
                    return Err(S3ObjectError::S3ObjectGetError {
                        object_key: object_key.to_string(),
                        error,
                    });
                }
            },
        };

        if let Some(content_md5_b64) = content_md5_b64 {
            let content_md5_bytes = base64::engine::general_purpose::STANDARD
                .decode(&content_md5_b64)
                .map_err(|error| S3ObjectError::ObjectETagB64Decode {
                    bucket_name: bucket_name.to_string(),
                    object_key: object_key.to_string(),
                    content_md5_b64,
                    error,
                })?;
            let content_md5_hexstr = content_md5_bytes
                .iter()
                .map(|x| format!("{:02x}", x))
                .collect::<String>();
            let state_current = S3ObjectState::Some {
                bucket_name: bucket_name.to_string(),
                object_key: object_key.to_string(),
                content_md5_hexstr,
            };

            Ok(state_current)
        } else {
            Ok(S3ObjectState::None)
        }
    }
}
