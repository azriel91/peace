use std::marker::PhantomData;

use peace::cfg::{async_trait, OpCtx, TryFnSpec};

use crate::item_specs::peace_aws_s3_bucket::{S3BucketData, S3BucketError, S3BucketState};

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressMsgUpdate;

/// Reads the current state of the S3 bucket state.
#[derive(Debug)]
pub struct S3BucketStateCurrentFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> TryFnSpec for S3BucketStateCurrentFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = S3BucketData<'op, Id>;
    type Error = S3BucketError;
    type Output = S3BucketState;

    async fn try_exec(
        op_ctx: OpCtx<'_>,
        data: S3BucketData<'_, Id>,
    ) -> Result<Option<Self::Output>, S3BucketError> {
        Self::exec(op_ctx, data).await.map(Some)
    }

    async fn exec(
        op_ctx: OpCtx<'_>,
        data: S3BucketData<'_, Id>,
    ) -> Result<Self::Output, S3BucketError> {
        let client = data.client();
        let name = data.params().name();

        #[cfg(not(feature = "output_progress"))]
        let _op_ctx = op_ctx;
        #[cfg(feature = "output_progress")]
        let progress_sender = &op_ctx.progress_sender;
        #[cfg(feature = "output_progress")]
        progress_sender.tick(ProgressMsgUpdate::Set(String::from("listing buckets")));
        let list_buckets_output = client.list_buckets().send().await.map_err(|error| {
            S3BucketError::S3BucketListError {
                s3_bucket_name: name.to_string(),
                error,
            }
        })?;
        #[cfg(feature = "output_progress")]
        progress_sender.inc(1, ProgressMsgUpdate::Set(String::from("finding bucket")));
        let s3_bucket_exists = list_buckets_output
            .buckets()
            .and_then(|buckets| {
                buckets.iter().find(|bucket| {
                    matches!(bucket.name(), Some(bucket_name_listed) if bucket_name_listed == name)
                })
            })
            .is_some();
        #[cfg(feature = "output_progress")]
        {
            let message = if s3_bucket_exists {
                "bucket found"
            } else {
                "bucket not found"
            };
            progress_sender.inc(1, ProgressMsgUpdate::Set(String::from(message)));
        }

        // let head_bucket_result = client.head_bucket().bucket(name).send().await;
        // let s3_bucket_exists = match head_bucket_result {
        //     Ok(_head_bucket_output) => true,
        //     Err(error) => match &error {
        //         SdkError::ServiceError(service_error) => {
        //             dbg!(&service_error);

        //             // If your user does not have permissions, AWS SDK Rust does not
        // return an             // access denied error. It just returns "Error"
        // with no other information.             //
        //             // https://github.com/awslabs/aws-sdk-rust/issues/227

        //             match service_error.err().kind {
        //                 HeadBucketErrorKind::NotFound(_) => false,
        //                 _ => {
        //                     return Err(S3BucketError::S3BucketGetError {
        //                         s3_bucket_name: name.to_string(),
        //                         error,
        //                     });
        //                 }
        //             }
        //         }
        //         _ => {
        //             return Err(S3BucketError::S3BucketGetError {
        //                 s3_bucket_name: name.to_string(),
        //                 error,
        //             });
        //         }
        //     },
        // };

        if s3_bucket_exists {
            let state_current = S3BucketState::Some {
                name: name.to_string(),
            };

            Ok(state_current)
        } else {
            Ok(S3BucketState::None)
        }
    }
}
