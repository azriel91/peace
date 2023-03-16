use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, CleanOpSpec, OpCheckStatus};

use crate::item_specs::peace_aws_s3_object::{S3ObjectData, S3ObjectError, S3ObjectState};

/// `CleanOpSpec` for the S3 object state.
#[derive(Debug, Default)]
pub struct S3ObjectCleanOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> CleanOpSpec for S3ObjectCleanOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = S3ObjectData<'op, Id>;
    type Error = S3ObjectError;
    type State = S3ObjectState;

    async fn check(
        _data: S3ObjectData<'_, Id>,
        state_current: &S3ObjectState,
    ) -> Result<OpCheckStatus, S3ObjectError> {
        let op_check_status = match state_current {
            S3ObjectState::None => OpCheckStatus::ExecNotRequired,
            S3ObjectState::Some {
                bucket_name: _,
                object_key: _,
                content_md5_hexstr: _,
            } => {
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
        _data: S3ObjectData<'_, Id>,
        _state_current: &S3ObjectState,
    ) -> Result<(), S3ObjectError> {
        Ok(())
    }

    async fn exec(
        data: S3ObjectData<'_, Id>,
        state_current: &S3ObjectState,
    ) -> Result<(), S3ObjectError> {
        match state_current {
            S3ObjectState::None => {}
            S3ObjectState::Some {
                bucket_name,
                object_key,
                content_md5_hexstr: _,
            } => {
                let client = data.client();
                client
                    .delete_object()
                    .bucket(bucket_name)
                    .key(object_key)
                    .send()
                    .await
                    .map_err(|error| {
                        let bucket_name = bucket_name.to_string();
                        let object_key = object_key.to_string();

                        S3ObjectError::S3ObjectDeleteError {
                            bucket_name,
                            object_key,
                            error,
                        }
                    })?;
            }
        };

        Ok(())
    }
}
