use std::marker::PhantomData;

use aws_sdk_iam::{error::ListPolicyVersionsErrorKind, types::SdkError};
#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, state::Generated, ApplyOpSpec, OpCheckStatus, OpCtx};

use crate::item_specs::peace_aws_iam_policy::{
    model::PolicyIdArnVersion, IamPolicyData, IamPolicyError, IamPolicyState, IamPolicyStateDiff,
};

use super::model::ManagedPolicyArn;

/// ApplyOpSpec for the instance profile state.
#[derive(Debug)]
pub struct IamPolicyApplyOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> ApplyOpSpec for IamPolicyApplyOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = IamPolicyData<'op, Id>;
    type Error = IamPolicyError;
    type State = IamPolicyState;
    type StateDiff = IamPolicyStateDiff;

    async fn check(
        mut data: IamPolicyData<'_, Id>,
        state_current: &IamPolicyState,
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
                let op_check_status = match state_current {
                    IamPolicyState::None => OpCheckStatus::ExecNotRequired,
                    IamPolicyState::Some {
                        name: _,
                        path: _,
                        policy_document: _,
                        policy_id_arn_version,
                    } => {
                        let mut steps_required = 0;
                        if matches!(policy_id_arn_version, Generated::Value(_)) {
                            steps_required += 1;
                        }

                        if steps_required == 0 {
                            OpCheckStatus::ExecNotRequired
                        } else {
                            #[cfg(not(feature = "output_progress"))]
                            {
                                OpCheckStatus::ExecRequired
                            }
                            #[cfg(feature = "output_progress")]
                            {
                                let progress_limit = ProgressLimit::Steps(steps_required);
                                OpCheckStatus::ExecRequired { progress_limit }
                            }
                        }
                    }
                };

                Ok(op_check_status)
            }
            IamPolicyStateDiff::NameOrPathModified {
                name_diff,
                path_diff,
            } => Err(IamPolicyError::PolicyModificationNotSupported {
                name_diff: name_diff.clone(),
                path_diff: path_diff.clone(),
            }),
            IamPolicyStateDiff::InSyncExists => {
                // Hack: Remove this when referential param values is implemented.
                let IamPolicyState::Some { policy_id_arn_version, .. } = state_current else {
                    unreachable!()
                };
                let Generated::Value(policy_id_version_arn) = policy_id_arn_version else {
                    unreachable!()
                };
                **data.managed_policy_arn_mut() = Some(ManagedPolicyArn::new(
                    policy_id_version_arn.arn().to_string(),
                ));

                Ok(OpCheckStatus::ExecNotRequired)
            }
            IamPolicyStateDiff::InSyncDoesNotExist => Ok(OpCheckStatus::ExecNotRequired),
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
        mut data: IamPolicyData<'_, Id>,
        state_current: &IamPolicyState,
        state_desired: &IamPolicyState,
        diff: &IamPolicyStateDiff,
    ) -> Result<IamPolicyState, IamPolicyError> {
        match diff {
            IamPolicyStateDiff::Added => match state_desired {
                IamPolicyState::None => {
                    panic!("`IamPolicyApplyOpSpec::exec` called with state_desired being None.");
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

                    // Hack: Remove this when referential param values is implemented.
                    **data.managed_policy_arn_mut() =
                        Some(ManagedPolicyArn::new(policy_arn.to_string()));

                    let policy_id_arn_version =
                        PolicyIdArnVersion::new(policy_id, policy_arn, policy_version);

                    let state_applied = IamPolicyState::Some {
                        name: name.to_string(),
                        path: path.clone(),
                        policy_document: policy_document.clone(),
                        policy_id_arn_version: Generated::Value(policy_id_arn_version),
                    };

                    Ok(state_applied)
                }
            },
            IamPolicyStateDiff::Removed => {
                match state_current {
                    IamPolicyState::None => {}
                    IamPolicyState::Some {
                        name,
                        path,
                        policy_document: _,
                        policy_id_arn_version,
                    } => {
                        if let Generated::Value(policy_id_arn_version) = policy_id_arn_version {
                            let client = data.client();
                            let list_policy_versions_result = client
                                .list_policy_versions()
                                .policy_arn(policy_id_arn_version.arn())
                                .send()
                                .await;

                            // Need to delete all of the non-default versions individually.
                            match list_policy_versions_result {
                                Ok(list_policy_versions_output) => {
                                    let policy_versions =
                                        list_policy_versions_output.versions().unwrap_or_default();

                                    let non_default_policy_versions =
                                        policy_versions.iter().filter(|policy_version| {
                                            !policy_version.is_default_version()
                                        });
                                    for policy_version in non_default_policy_versions {
                                        let version_id = policy_version.version_id().expect(
                                            "Expected policy version version ID to be Some.",
                                        );
                                        client
                                            .delete_policy_version()
                                            .policy_arn(policy_id_arn_version.arn())
                                            .version_id(version_id)
                                            .send()
                                            .await
                                            .map_err(|error| {
                                                IamPolicyError::PolicyVersionDeleteError {
                                                    policy_name: name.to_string(),
                                                    policy_path: path.to_string(),
                                                    version: version_id.to_string(),
                                                    error,
                                                }
                                            })?;
                                    }
                                }
                                Err(error) => match &error {
                                    SdkError::ServiceError(service_error) => match service_error
                                        .err()
                                        .kind
                                    {
                                        ListPolicyVersionsErrorKind::NoSuchEntityException(_) => {
                                            return Err(IamPolicyError::PolicyNotFoundAfterList {
                                                policy_name: name.to_string(),
                                                policy_path: path.to_string(),
                                                policy_id: policy_id_arn_version.id().to_string(),
                                                policy_arn: policy_id_arn_version.arn().to_string(),
                                            });
                                        }
                                        _ => {
                                            return Err(IamPolicyError::PolicyVersionsListError {
                                                policy_name: name.to_string(),
                                                policy_path: path.to_string(),
                                                error,
                                            });
                                        }
                                    },
                                    _ => {
                                        return Err(IamPolicyError::PolicyVersionsListError {
                                            policy_name: name.to_string(),
                                            policy_path: path.to_string(),
                                            error,
                                        });
                                    }
                                },
                            };

                            // The default version is deleted along with the policy.
                            client
                                .delete_policy()
                                .policy_arn(policy_id_arn_version.arn())
                                .send()
                                .await
                                .map_err(|error| {
                                    let policy_name = name.to_string();
                                    let policy_path = path.to_string();
                                    let policy_id = policy_id_arn_version.id().to_string();
                                    let policy_arn = policy_id_arn_version.arn().to_string();

                                    IamPolicyError::PolicyDeleteError {
                                        policy_name,
                                        policy_path,
                                        policy_id,
                                        policy_arn,
                                        error,
                                    }
                                })?;
                        }
                    }
                }

                let state_applied = state_desired.clone();
                Ok(state_applied)
            }
            IamPolicyStateDiff::DocumentModified { .. } => match state_desired {
                IamPolicyState::None => {
                    panic!("`IamPolicyApplyOpSpec::exec` called with state_desired being None.");
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

                    // Hack: Remove this when referential param values is implemented.
                    **data.managed_policy_arn_mut() =
                        Some(ManagedPolicyArn::new(policy_arn.to_string()));

                    let policy_id_arn_version =
                        PolicyIdArnVersion::new(policy_id, policy_arn, policy_version_id);

                    let state_applied = IamPolicyState::Some {
                        name: name.to_string(),
                        path: path.clone(),
                        policy_document: policy_document.clone(),
                        policy_id_arn_version: Generated::Value(policy_id_arn_version),
                    };

                    Ok(state_applied)
                }
            },
            IamPolicyStateDiff::InSyncExists | IamPolicyStateDiff::InSyncDoesNotExist => {
                unreachable!(
                    "`IamPolicyApplyOpSpec::exec` should never be called when state is in sync."
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
