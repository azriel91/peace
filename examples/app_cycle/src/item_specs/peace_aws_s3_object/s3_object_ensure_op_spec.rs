use std::marker::PhantomData;

use aws_sdk_s3::types::ByteStream;
use base64::Engine;
#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, state::Generated, EnsureOpSpec, OpCheckStatus, OpCtx};

use crate::item_specs::peace_aws_s3_object::{
    S3ObjectData, S3ObjectError, S3ObjectState, S3ObjectStateDiff,
};

/// Ensure OpSpec for the S3 object state.
#[derive(Debug)]
pub struct S3ObjectEnsureOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> EnsureOpSpec for S3ObjectEnsureOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = S3ObjectData<'op, Id>;
    type Error = S3ObjectError;
    type State = S3ObjectState;
    type StateDiff = S3ObjectStateDiff;

    async fn check(
        _s3_object_data: S3ObjectData<'_, Id>,
        _state_current: &S3ObjectState,
        _state_desired: &S3ObjectState,
        diff: &S3ObjectStateDiff,
    ) -> Result<OpCheckStatus, S3ObjectError> {
        match diff {
            S3ObjectStateDiff::Added { .. } | S3ObjectStateDiff::ObjectContentModified { .. } => {
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
            S3ObjectStateDiff::Removed => {
                panic!(
                    "`S3ObjectEnsureOpSpec::check` called with `S3ObjectStateDiff::Removed`.\n\
                    An ensure should never remove a object."
                );
            }
            S3ObjectStateDiff::BucketNameModified {
                bucket_name_current,
                bucket_name_desired,
            } => Err(S3ObjectError::BucketModificationNotSupported {
                bucket_name_current: bucket_name_current.clone(),
                bucket_name_desired: bucket_name_desired.clone(),
            }),
            S3ObjectStateDiff::ObjectKeyModified {
                object_key_current,
                object_key_desired,
            } => Err(S3ObjectError::S3ObjectModificationNotSupported {
                object_key_current: object_key_current.clone(),
                object_key_desired: object_key_desired.clone(),
            }),
            S3ObjectStateDiff::InSyncExists | S3ObjectStateDiff::InSyncDoesNotExist => {
                Ok(OpCheckStatus::ExecNotRequired)
            }
        }
    }

    async fn exec_dry(
        _op_ctx: OpCtx<'_>,
        _s3_object_data: S3ObjectData<'_, Id>,
        _state_current: &S3ObjectState,
        state_desired: &S3ObjectState,
        _diff: &S3ObjectStateDiff,
    ) -> Result<S3ObjectState, S3ObjectError> {
        Ok(state_desired.clone())
    }

    async fn exec(
        _op_ctx: OpCtx<'_>,
        data: S3ObjectData<'_, Id>,
        _state_current: &S3ObjectState,
        state_desired: &S3ObjectState,
        diff: &S3ObjectStateDiff,
    ) -> Result<S3ObjectState, S3ObjectError> {
        match diff {
            S3ObjectStateDiff::Added | S3ObjectStateDiff::ObjectContentModified { .. } => {
                match state_desired {
                    S3ObjectState::None => {
                        panic!(
                            "`S3ObjectEnsureOpSpec::exec` called with state_desired being None."
                        );
                    }
                    S3ObjectState::Some {
                        bucket_name,
                        object_key,
                        content_md5_hexstr,
                        e_tag: _,
                    } => {
                        let client = data.client();
                        let file_path = data.params().file_path();
                        let Some(content_md5_hexstr) = content_md5_hexstr else {
                            panic!("Content MD5 must be Some as this is calculated from an existent local file.");
                        };
                        let content_md5_b64 = {
                            let bytes = (0..content_md5_hexstr.len())
                                .step_by(2)
                                .map(|index_start| {
                                    &content_md5_hexstr[index_start..index_start + 2]
                                })
                                .map(|byte_hexstr| u8::from_str_radix(byte_hexstr, 16))
                                .try_fold(
                                    Vec::<u8>::with_capacity(content_md5_hexstr.len() / 2),
                                    |mut bytes, byte_result| {
                                        byte_result.map(|byte| {
                                            bytes.push(byte);
                                            bytes
                                        })
                                    },
                                )
                                .map_err(|error| {
                                    let file_path = file_path.to_path_buf();
                                    let bucket_name = bucket_name.clone();
                                    let object_key = object_key.clone();
                                    let content_md5_hexstr = content_md5_hexstr.clone();

                                    S3ObjectError::ObjectContentMd5HexstrParse {
                                        file_path,
                                        bucket_name,
                                        object_key,
                                        content_md5_hexstr,
                                        error,
                                    }
                                })?;
                            base64::engine::general_purpose::STANDARD.encode(bytes)
                        };
                        let put_object_output = client
                            .put_object()
                            .bucket(bucket_name)
                            .key(object_key)
                            .content_md5(content_md5_b64)
                            .metadata("content_md5_hexstr", content_md5_hexstr)
                            .body(ByteStream::from_path(file_path).await.map_err(|error| {
                                let file_path = file_path.to_path_buf();
                                let bucket_name = bucket_name.clone();
                                let object_key = object_key.clone();

                                S3ObjectError::ObjectFileStream {
                                    file_path,
                                    bucket_name,
                                    object_key,
                                    error,
                                }
                            })?)
                            .send()
                            .await
                            .map_err(|error| {
                                let bucket_name = bucket_name.to_string();
                                let object_key = object_key.to_string();
                                S3ObjectError::S3ObjectUploadError {
                                    bucket_name,
                                    object_key,
                                    error,
                                }
                            })?;
                        let e_tag = put_object_output
                            .e_tag()
                            .expect("Expected ETag to be some when put_object is successful.")
                            .to_string();

                        let state_ensured = S3ObjectState::Some {
                            bucket_name: bucket_name.clone(),
                            object_key: object_key.clone(),
                            content_md5_hexstr: Some(content_md5_hexstr.clone()),
                            e_tag: Generated::Value(e_tag),
                        };

                        Ok(state_ensured)
                    }
                }
            }
            S3ObjectStateDiff::Removed => {
                panic!(
                    "`S3ObjectEnsureOpSpec::exec` called with `S3ObjectStateDiff::Removed`.\n\
                    An ensure should never remove a object."
                );
            }
            S3ObjectStateDiff::InSyncExists | S3ObjectStateDiff::InSyncDoesNotExist => {
                unreachable!(
                    "`S3ObjectEnsureOpSpec::exec` should never be called when state is in sync."
                );
            }
            S3ObjectStateDiff::BucketNameModified {
                bucket_name_current,
                bucket_name_desired,
            } => Err(S3ObjectError::BucketModificationNotSupported {
                bucket_name_current: bucket_name_current.clone(),
                bucket_name_desired: bucket_name_desired.clone(),
            }),
            S3ObjectStateDiff::ObjectKeyModified {
                object_key_current,
                object_key_desired,
            } => {
                let S3ObjectState::Some {bucket_name, ..} = state_desired else {
                    panic!("`S3ObjectEnsureOpSpec::exec` called with state_desired being None.");
                };

                Err(S3ObjectError::ObjectKeyModificationNotSupported {
                    bucket_name: bucket_name.clone(),
                    object_key_current: object_key_current.clone(),
                    object_key_desired: object_key_desired.clone(),
                })
            }
        }
    }
}
