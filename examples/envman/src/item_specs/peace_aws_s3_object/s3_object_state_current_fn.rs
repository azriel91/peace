use std::marker::PhantomData;

use aws_sdk_iam::types::SdkError;
use aws_sdk_s3::error::HeadObjectErrorKind;
use peace::cfg::{state::Generated, OpCtx};

use crate::item_specs::peace_aws_s3_object::{S3ObjectData, S3ObjectError, S3ObjectState};

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressMsgUpdate;

/// Reads the current state of the S3 object state.
#[derive(Debug)]
pub struct S3ObjectStateCurrentFn<Id>(PhantomData<Id>);

impl<Id> S3ObjectStateCurrentFn<Id>
where
    Id: Send + Sync,
{
    pub async fn try_state_current(
        op_ctx: OpCtx<'_>,
        data: S3ObjectData<'_, Id>,
    ) -> Result<Option<S3ObjectState>, S3ObjectError> {
        Self::state_current(op_ctx, data).await.map(Some)
    }

    pub async fn state_current(
        op_ctx: OpCtx<'_>,
        data: S3ObjectData<'_, Id>,
    ) -> Result<S3ObjectState, S3ObjectError> {
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

                #[cfg(feature = "error_reporting")]
                let (aws_desc, aws_desc_span) = crate::item_specs::aws_error_desc!(&error);

                match &error {
                    SdkError::ServiceError(service_error) => match service_error.err().kind {
                        HeadObjectErrorKind::NotFound(_) => None,
                        _ => {
                            return Err(S3ObjectError::S3ObjectGetError {
                                object_key: object_key.to_string(),
                                #[cfg(feature = "error_reporting")]
                                aws_desc,
                                #[cfg(feature = "error_reporting")]
                                aws_desc_span,
                                error,
                            });
                        }
                    },
                    _ => {
                        return Err(S3ObjectError::S3ObjectGetError {
                            object_key: object_key.to_string(),
                            #[cfg(feature = "error_reporting")]
                            aws_desc,
                            #[cfg(feature = "error_reporting")]
                            aws_desc_span,
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