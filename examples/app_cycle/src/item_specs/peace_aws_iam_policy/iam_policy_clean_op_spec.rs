use std::marker::PhantomData;

use aws_sdk_iam::{error::ListPolicyVersionsErrorKind, types::SdkError};
#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, state::Generated, CleanOpSpec, OpCheckStatus};

use crate::item_specs::peace_aws_iam_policy::{IamPolicyData, IamPolicyError, IamPolicyState};

/// `CleanOpSpec` for the instance profile state.
#[derive(Debug, Default)]
pub struct IamPolicyCleanOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> CleanOpSpec for IamPolicyCleanOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = IamPolicyData<'op, Id>;
    type Error = IamPolicyError;
    type State = IamPolicyState;

    async fn check(
        _data: IamPolicyData<'_, Id>,
        state_current: &IamPolicyState,
    ) -> Result<OpCheckStatus, IamPolicyError> {
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

    async fn exec_dry(
        _data: IamPolicyData<'_, Id>,
        _state_current: &IamPolicyState,
    ) -> Result<(), IamPolicyError> {
        Ok(())
    }

    async fn exec(
        data: IamPolicyData<'_, Id>,
        state_current: &IamPolicyState,
    ) -> Result<(), IamPolicyError> {
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

                            let non_default_policy_versions = policy_versions
                                .iter()
                                .filter(|policy_version| !policy_version.is_default_version());
                            for policy_version in non_default_policy_versions {
                                let version_id = policy_version
                                    .version_id()
                                    .expect("Expected policy version version ID to be Some.");
                                client
                                    .delete_policy_version()
                                    .policy_arn(policy_id_arn_version.arn())
                                    .version_id(version_id)
                                    .send()
                                    .await
                                    .map_err(|error| IamPolicyError::PolicyVersionDeleteError {
                                        policy_name: name.to_string(),
                                        policy_path: path.to_string(),
                                        version: version_id.to_string(),
                                        error,
                                    })?;
                            }
                        }
                        Err(error) => match &error {
                            SdkError::ServiceError(service_error) => match service_error.err().kind
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
        };

        Ok(())
    }
}
