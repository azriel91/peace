use std::marker::PhantomData;

use aws_sdk_iam::types::SdkError;
use aws_sdk_s3::error::HeadObjectErrorKind;
use peace::cfg::{async_trait, state::Generated, OpCtx, TryFnSpec};

use crate::item_specs::peace_aws_s3_object::{S3ObjectData, S3ObjectError, S3ObjectState};

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressMsgUpdate;

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

    async fn try_exec(
        op_ctx: OpCtx<'_>,
        data: S3ObjectData<'_, Id>,
    ) -> Result<Option<Self::Output>, S3ObjectError> {
        Self::exec(op_ctx, data).await.map(Some)
    }

    async fn exec(
        op_ctx: OpCtx<'_>,
        data: S3ObjectData<'_, Id>,
    ) -> Result<Self::Output, S3ObjectError> {
        let client = data.client();
        let bucket_name = data.params().bucket_name();
        let object_key = data.params().object_key();

        #[cfg(not(feature = "output_progress"))]
        let _op_ctx = op_ctx;
        #[cfg(feature = "output_progress")]
        let progress_sender = &op_ctx.progress_sender;
        #[cfg(feature = "output_progress")]
        progress_sender.tick(ProgressMsgUpdate::Set(String::from(
            "fetching object metadata",
        )));
        let head_object_result = client
            .head_object()
            .bucket(bucket_name)
            .key(object_key)
            .send()
            .await;
        let content_md5_and_e_tag = match head_object_result {
            Ok(head_object_output) => {
                #[cfg(feature = "output_progress")]
                progress_sender.tick(ProgressMsgUpdate::Set(String::from(
                    "object metadata fetched",
                )));
                let content_md5_hexstr = head_object_output
                    .metadata()
                    .and_then(|metadata| metadata.get("content_md5_hexstr"))
                    .cloned();

                let e_tag = head_object_output
                    .e_tag()
                    .expect("Expected S3 object e_tag to be Some when head_object is successful.")
                    .trim_matches('"')
                    .to_string();

                Some((content_md5_hexstr, e_tag))
            }
            Err(error) => {
                #[cfg(feature = "output_progress")]
                progress_sender.tick(ProgressMsgUpdate::Set(String::from(
                    "object metadata not fetched",
                )));
                match &error {
                    SdkError::ServiceError(service_error) => match service_error.err().kind {
                        HeadObjectErrorKind::NotFound(_) => None,
                        _ => {
                            return Err(S3ObjectError::S3ObjectGetError {
                                object_key: object_key.to_string(),
                                error,
                            });
                        }
                    },
                    _ => {
                        return Err(S3ObjectError::S3ObjectGetError {
                            object_key: object_key.to_string(),
                            error,
                        });
                    }
                }
            }
        };

        if let Some((content_md5_hexstr, e_tag)) = content_md5_and_e_tag {
            let state_current = S3ObjectState::Some {
                bucket_name: bucket_name.to_string(),
                object_key: object_key.to_string(),
                content_md5_hexstr,
                e_tag: Generated::Value(e_tag),
            };

            Ok(state_current)
        } else {
            Ok(S3ObjectState::None)
        }
    }
}
