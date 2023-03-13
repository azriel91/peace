use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, state::Generated, EnsureOpSpec, OpCheckStatus, OpCtx};

use crate::item_specs::peace_aws_iam_policy::{
    model::PolicyIdArnVersion, IamPolicyData, IamPolicyError, IamPolicyState, IamPolicyStateDiff,
};

/// Ensure OpSpec for the instance profile state.
#[derive(Debug)]
pub struct IamPolicyEnsureOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> EnsureOpSpec for IamPolicyEnsureOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = IamPolicyData<'op, Id>;
    type Error = IamPolicyError;
    type State = IamPolicyState;
    type StateDiff = IamPolicyStateDiff;

    async fn check(
        _iam_policy_data: IamPolicyData<'_, Id>,
        _state_current: &IamPolicyState,
        _state_desired: &IamPolicyState,
        diff: &IamPolicyStateDiff,
    ) -> Result<OpCheckStatus, IamPolicyError> {
        match diff {
            IamPolicyStateDiff::Added | IamPolicyStateDiff::DocumentModified { .. } => {
                let op_check_status = {
                    #[cfg(not(feature = "output_progress"))]
                    {
                        OpCheckStatus::ExecRequired
                    }
                    #[cfg(feature = "output_progress")]
                    {
                        let progress_limit = ProgressLimit::Steps(3);
                        OpCheckStatus::ExecRequired { progress_limit }
                    }
                };

                Ok(op_check_status)
            }
            IamPolicyStateDiff::Removed => {
                panic!(
                    "`IamPolicyEnsureOpSpec::check` called with `IamPolicyStateDiff::Removed`.\n\
                    An ensure should never remove an instance profile."
                );
            }
            IamPolicyStateDiff::NameOrPathModified {
                name_diff,
                path_diff,
            } => Err(IamPolicyError::PolicyModificationNotSupported {
                name_diff: name_diff.clone(),
                path_diff: path_diff.clone(),
            }),
            IamPolicyStateDiff::InSyncExists | IamPolicyStateDiff::InSyncDoesNotExist => {
                Ok(OpCheckStatus::ExecNotRequired)
            }
        }
    }

    async fn exec_dry(
        _op_ctx: OpCtx<'_>,
        _iam_policy_data: IamPolicyData<'_, Id>,
        _state_current: &IamPolicyState,
        state_desired: &IamPolicyState,
        _diff: &IamPolicyStateDiff,
    ) -> Result<IamPolicyState, IamPolicyError> {
        Ok(state_desired.clone())
    }

    async fn exec(
        _op_ctx: OpCtx<'_>,
        data: IamPolicyData<'_, Id>,
        state_current: &IamPolicyState,
        state_desired: &IamPolicyState,
        diff: &IamPolicyStateDiff,
    ) -> Result<IamPolicyState, IamPolicyError> {
        match diff {
            IamPolicyStateDiff::Added => match state_desired {
                IamPolicyState::None => {
                    panic!("`IamPolicyEnsureOpSpec::exec` called with state_desired being None.");
                }
                IamPolicyState::Some {
                    name,
                    path,
                    policy_document,
                    policy_id_arn_version: _,
                } => {
                    let create_policy_output = data
                        .client()
                        .create_policy()
                        .policy_name(name)
                        .path(path)
                        .policy_document(policy_document)
                        .send()
                        .await
                        .map_err(|error| {
                            let policy_name = name.to_string();
                            let policy_path = path.to_string();

                            IamPolicyError::PolicyCreateError {
                                policy_name,
                                policy_path,
                                error,
                            }
                        })?;
                    let policy = create_policy_output
                        .policy()
                        .expect("Expected policy to be Some when create_policy is successful.");
                    let policy_id = policy
                        .policy_id()
                        .expect("Expected policy id to be Some when create_policy is successful.")
                        .to_string();
                    let policy_arn = policy
                        .arn()
                        .expect("Expected policy ARN to be Some when create_policy is successful.")
                        .to_string();
                    let policy_version = policy
                        .default_version_id()
                        .expect(
                            "Expected policy default version ID to be Some \
                            when create_policy is successful.",
                        )
                        .to_string();
                    let policy_id_arn_version =
                        PolicyIdArnVersion::new(policy_id, policy_arn, policy_version);

                    let state_ensured = IamPolicyState::Some {
                        name: name.to_string(),
                        path: path.clone(),
                        policy_document: policy_document.clone(),
                        policy_id_arn_version: Generated::Value(policy_id_arn_version),
                    };

                    Ok(state_ensured)
                }
            },
            IamPolicyStateDiff::Removed => {
                panic!(
                    "`IamPolicyEnsureOpSpec::exec` called with `IamPolicyStateDiff::Removed`.\n\
                    An ensure should never remove an instance profile."
                );
            }
            IamPolicyStateDiff::DocumentModified { .. } => match state_desired {
                IamPolicyState::None => {
                    panic!("`IamPolicyEnsureOpSpec::exec` called with state_desired being None.");
                }
                IamPolicyState::Some {
                    name,
                    path,
                    policy_document,
                    policy_id_arn_version: _,
                } => {
                    let IamPolicyState::Some { policy_id_arn_version: Generated::Value(policy_id_arn_version), .. } = state_current else {
                        panic!("Expected policy ID and ARN to exist when diff is modified.");
                    };
                    let policy_arn = policy_id_arn_version.arn();
                    let create_policy_output = data
                        .client()
                        .create_policy_version()
                        .policy_arn(policy_arn)
                        .policy_document(policy_document)
                        .send()
                        .await
                        .map_err(|error| {
                            let policy_name = name.to_string();
                            let policy_path = path.to_string();

                            IamPolicyError::PolicyVersionCreateError {
                                policy_name,
                                policy_path,
                                error,
                            }
                        })?;
                    let policy_version = create_policy_output.policy_version().expect(
                        "Expected policy_version to be Some when create_policy is successful.",
                    );
                    let policy_id = policy_id_arn_version.id().to_string();
                    let policy_arn = policy_id_arn_version.arn().to_string();
                    let policy_version_id = policy_version
                        .version_id()
                        .expect("Expected policy_version version_id to be Some when create_policy is successful.")
                        .to_string();
                    let policy_id_arn_version =
                        PolicyIdArnVersion::new(policy_id, policy_arn, policy_version_id);

                    let state_ensured = IamPolicyState::Some {
                        name: name.to_string(),
                        path: path.clone(),
                        policy_document: policy_document.clone(),
                        policy_id_arn_version: Generated::Value(policy_id_arn_version),
                    };

                    Ok(state_ensured)
                }
            },
            IamPolicyStateDiff::InSyncExists | IamPolicyStateDiff::InSyncDoesNotExist => {
                unreachable!(
                    "`IamPolicyEnsureOpSpec::exec` should never be called when state is in sync."
                );
            }
            IamPolicyStateDiff::NameOrPathModified {
                name_diff,
                path_diff,
            } => Err(IamPolicyError::NameOrPathModificationNotSupported {
                name_diff: name_diff.clone(),
                path_diff: path_diff.clone(),
            }),
        }
    }
}
