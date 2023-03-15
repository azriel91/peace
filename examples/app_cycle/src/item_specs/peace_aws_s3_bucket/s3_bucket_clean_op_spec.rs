use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, CleanOpSpec, OpCheckStatus};

use crate::item_specs::peace_aws_s3_bucket::{S3BucketData, S3BucketError, S3BucketState};

/// `CleanOpSpec` for the S3 bucket state.
#[derive(Debug, Default)]
pub struct S3BucketCleanOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> CleanOpSpec for S3BucketCleanOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = S3BucketData<'op, Id>;
    type Error = S3BucketError;
    type State = S3BucketState;

    async fn check(
        _data: S3BucketData<'_, Id>,
        state_current: &S3BucketState,
    ) -> Result<OpCheckStatus, S3BucketError> {
        let op_check_status = match state_current {
            S3BucketState::None => OpCheckStatus::ExecNotRequired,
            S3BucketState::Some { name: _ } => {
                #[cfg(not(feature = "output_progress"))]
                {
                    OpCheckStatus::ExecRequired
                }
                #[cfg(feature = "output_progress")]
                {
                    let steps_required = 1;
                    let progress_limit = ProgressLimit::Steps(steps_required);
                    OpCheckStatus::ExecRequired { progress_limit }
                }
            }
        };

        Ok(op_check_status)
    }

    async fn exec_dry(
        _data: S3BucketData<'_, Id>,
        _state_current: &S3BucketState,
    ) -> Result<(), S3BucketError> {
        Ok(())
    }

    async fn exec(
        data: S3BucketData<'_, Id>,
        state_current: &S3BucketState,
    ) -> Result<(), S3BucketError> {
        match state_current {
            S3BucketState::None => {}
            S3BucketState::Some { name } => {
                let client = data.client();
                client
                    .delete_bucket()
                    .bucket(name)
                    .send()
                    .await
                    .map_err(|error| {
                        let s3_bucket_name = name.to_string();

                        S3BucketError::S3BucketDeleteError {
                            s3_bucket_name,
                            error,
                        }
                    })?;
            }
        };

        Ok(())
    }
}
