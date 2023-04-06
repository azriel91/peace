use std::marker::PhantomData;

use aws_sdk_iam::types::SdkError;
use aws_sdk_s3::{
    error::CreateBucketErrorKind,
    model::{BucketLocationConstraint, CreateBucketConfiguration},
};
#[cfg(feature = "output_progress")]
use peace::cfg::progress::{ProgressLimit, ProgressMsgUpdate};
use peace::cfg::{async_trait, ApplyOpSpec, OpCheckStatus, OpCtx};

use crate::item_specs::peace_aws_s3_bucket::{
    S3BucketData, S3BucketError, S3BucketState, S3BucketStateDiff,
};

use super::S3BucketStateCurrentFnSpec;

/// ApplyOpSpec for the S3 bucket state.
#[derive(Debug)]
pub struct S3BucketApplyOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> ApplyOpSpec for S3BucketApplyOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = S3BucketData<'op, Id>;
    type Error = S3BucketError;
    type State = S3BucketState;
    type StateDiff = S3BucketStateDiff;

    async fn check(
        _s3_bucket_data: S3BucketData<'_, Id>,
        state_current: &S3BucketState,
        _state_desired: &S3BucketState,
        diff: &S3BucketStateDiff,
    ) -> Result<OpCheckStatus, S3BucketError> {
        match diff {
            S3BucketStateDiff::Added { .. } => {
                let op_check_status = {
                    #[cfg(not(feature = "output_progress"))]
                    {
                        OpCheckStatus::ExecRequired
                    }
                    #[cfg(feature = "output_progress")]
                    {
                        let progress_limit = ProgressLimit::Steps(1);
                        OpCheckStatus::ExecRequired { progress_limit }
                    }
                };

                Ok(op_check_status)
            }
            S3BucketStateDiff::Removed => {
                let op_check_status = match state_current {
                    S3BucketState::None => OpCheckStatus::ExecNotRequired,
                    S3BucketState::Some {
                        name: _,
                        creation_date: _,
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
            S3BucketStateDiff::NameModified {
                s3_bucket_name_current,
                s3_bucket_name_desired,
            } => Err(S3BucketError::S3BucketModificationNotSupported {
                s3_bucket_name_current: s3_bucket_name_current.clone(),
                s3_bucket_name_desired: s3_bucket_name_desired.clone(),
            }),
            S3BucketStateDiff::InSyncExists | S3BucketStateDiff::InSyncDoesNotExist => {
                Ok(OpCheckStatus::ExecNotRequired)
            }
        }
    }

    async fn exec_dry(
        _op_ctx: OpCtx<'_>,
        _s3_bucket_data: S3BucketData<'_, Id>,
        _state_current: &S3BucketState,
        state_desired: &S3BucketState,
        _diff: &S3BucketStateDiff,
    ) -> Result<S3BucketState, S3BucketError> {
        Ok(state_desired.clone())
    }

    // Not sure why we can't use this:
    //
    // #[cfg(not(feature = "output_progress"))] _op_ctx: OpCtx<'_>,
    // #[cfg(feature = "output_progress")] op_ctx: OpCtx<'_>,
    //
    // There's an error saying lifetime bounds don't match the trait definition.
    //
    // Likely an issue with the codegen in `async-trait`.
    #[allow(unused_variables)]
    async fn exec(
        op_ctx: OpCtx<'_>,
        data: S3BucketData<'_, Id>,
        state_current: &S3BucketState,
        state_desired: &S3BucketState,
        diff: &S3BucketStateDiff,
    ) -> Result<S3BucketState, S3BucketError> {
        #[cfg(feature = "output_progress")]
        let progress_sender = &op_ctx.progress_sender;

        match diff {
            S3BucketStateDiff::Added => match state_desired {
                S3BucketState::None => {
                    panic!("`S3BucketApplyOpSpec::exec` called with state_desired being None.");
                }
                S3BucketState::Some {
                    name,
                    creation_date: _,
                } => {
                    let client = data.client();

                    #[cfg(feature = "output_progress")]
                    progress_sender.tick(ProgressMsgUpdate::Set(String::from("creating bucket")));
                    let mut create_bucket = client.create_bucket().bucket(name);
                    if let Some(region) = data.region().as_ref() {
                        create_bucket = create_bucket.create_bucket_configuration(
                            CreateBucketConfiguration::builder()
                                .location_constraint(BucketLocationConstraint::from(
                                    region.as_ref(),
                                ))
                                .build(),
                        );
                    }
                    let create_bucket_output = create_bucket.send().await.map_err(|error| {
                        let s3_bucket_name = name.to_string();

                        #[cfg(feature = "error_reporting")]
                        let (aws_desc, aws_desc_span) = crate::item_specs::aws_error_desc!(&error);

                        match &error {
                            SdkError::ServiceError(service_error) => {
                                match &service_error.err().kind {
                                    CreateBucketErrorKind::BucketAlreadyExists(error) => {
                                        S3BucketError::S3BucketOwnedBySomeoneElseError {
                                            s3_bucket_name,
                                            error: error.clone(),
                                        }
                                    }
                                    CreateBucketErrorKind::BucketAlreadyOwnedByYou(error) => {
                                        S3BucketError::S3BucketOwnedByYouError {
                                            s3_bucket_name,
                                            error: error.clone(),
                                        }
                                    }
                                    _ => S3BucketError::S3BucketCreateError {
                                        s3_bucket_name,
                                        #[cfg(feature = "error_reporting")]
                                        aws_desc,
                                        #[cfg(feature = "error_reporting")]
                                        aws_desc_span,
                                        error,
                                    },
                                }
                            }
                            _ => S3BucketError::S3BucketCreateError {
                                s3_bucket_name,
                                #[cfg(feature = "error_reporting")]
                                aws_desc,
                                #[cfg(feature = "error_reporting")]
                                aws_desc_span,
                                error,
                            },
                        }
                    })?;
                    #[cfg(feature = "output_progress")]
                    progress_sender.inc(1, ProgressMsgUpdate::Set(String::from("bucket created")));

                    let state_applied =
                        S3BucketStateCurrentFnSpec::<Id>::state_current(op_ctx, data).await?;

                    Ok(state_applied)
                }
            },
            S3BucketStateDiff::Removed => {
                match state_current {
                    S3BucketState::None => {}
                    S3BucketState::Some {
                        name,
                        creation_date: _,
                    } => {
                        let client = data.client();

                        #[cfg(feature = "output_progress")]
                        progress_sender
                            .tick(ProgressMsgUpdate::Set(String::from("deleting bucket")));
                        let delete_bucket_result = client.delete_bucket().bucket(name).send().await;

                        // Sometimes AWS returns this error:
                        //
                        // ```xml
                        // <Code>NoSuchBucket</Code>
                        // <Message>The specified bucket does not exist</Message>
                        // <BucketName>the-bucket-name</BucketName>
                        // ```
                        //
                        // This is really an issue with AWS, where they still show the
                        // bucket even though it *has* been deleted. See:
                        //
                        // <https://serverfault.com/questions/969962/how-to-remove-orphaned-s3-bucket>
                        delete_bucket_result
                            .map(|_delete_bucket_output| ())
                            .or_else(|error| {
                                if let SdkError::ServiceError(service_error) = &error {
                                    if let Some("NoSuchBucket") = service_error.err().code() {
                                        return Ok(());
                                    }
                                }

                                #[cfg(feature = "error_reporting")]
                                let (aws_desc, aws_desc_span) =
                                    crate::item_specs::aws_error_desc!(&error);

                                let s3_bucket_name = name.to_string();
                                Err(S3BucketError::S3BucketDeleteError {
                                    s3_bucket_name,
                                    #[cfg(feature = "error_reporting")]
                                    aws_desc,
                                    #[cfg(feature = "error_reporting")]
                                    aws_desc_span,
                                    error,
                                })
                            })?;
                        #[cfg(feature = "output_progress")]
                        progress_sender
                            .inc(1, ProgressMsgUpdate::Set(String::from("bucket deleted")));
                    }
                }

                let state_applied = state_desired.clone();
                Ok(state_applied)
            }
            S3BucketStateDiff::InSyncExists | S3BucketStateDiff::InSyncDoesNotExist => {
                unreachable!(
                    "`S3BucketApplyOpSpec::exec` should never be called when state is in sync."
                );
            }
            S3BucketStateDiff::NameModified {
                s3_bucket_name_current,
                s3_bucket_name_desired,
            } => Err(S3BucketError::NameModificationNotSupported {
                s3_bucket_name_current: s3_bucket_name_current.clone(),
                s3_bucket_name_desired: s3_bucket_name_desired.clone(),
            }),
        }
    }
}
