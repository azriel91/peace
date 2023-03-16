use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, state::Generated, ApplyOpSpec, OpCheckStatus, OpCtx};

use crate::item_specs::peace_aws_instance_profile::{
    model::InstanceProfileIdAndArn, InstanceProfileCleanOpSpec, InstanceProfileData,
    InstanceProfileError, InstanceProfileState, InstanceProfileStateDiff,
};

/// ApplyOpSpec for the instance profile state.
#[derive(Debug)]
pub struct InstanceProfileApplyOpSpec<Id>(PhantomData<Id>);

impl<Id> InstanceProfileApplyOpSpec<Id> {
    async fn role_associate(
        client: &aws_sdk_iam::Client,
        name: &str,
        path: &str,
    ) -> Result<(), InstanceProfileError> {
        let _instance_profile_role_add_output = client
            .add_role_to_instance_profile()
            .role_name(name)
            .instance_profile_name(name)
            .send()
            .await
            .map_err(|error| {
                let instance_profile_name = name.to_string();
                let instance_profile_path = path.to_string();
                let role_name = name.to_string();

                InstanceProfileError::InstanceProfileRoleAddError {
                    instance_profile_name,
                    instance_profile_path,
                    role_name,
                    error,
                }
            })?;

        Ok(())
    }
}

#[async_trait(?Send)]
impl<Id> ApplyOpSpec for InstanceProfileApplyOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = InstanceProfileData<'op, Id>;
    type Error = InstanceProfileError;
    type State = InstanceProfileState;
    type StateDiff = InstanceProfileStateDiff;

    async fn check(
        _instance_profile_data: InstanceProfileData<'_, Id>,
        _state_current: &InstanceProfileState,
        _state_desired: &InstanceProfileState,
        diff: &InstanceProfileStateDiff,
    ) -> Result<OpCheckStatus, InstanceProfileError> {
        match diff {
            InstanceProfileStateDiff::Added
            | InstanceProfileStateDiff::RoleAssociatedModified { .. } => {
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
            InstanceProfileStateDiff::Removed => {
                panic!(
                    "`InstanceProfileApplyOpSpec::check` called with `InstanceProfileStateDiff::Removed`.\n\
                    An ensure should never remove an instance profile."
                );
            }
            InstanceProfileStateDiff::NameOrPathModified {
                name_diff,
                path_diff,
            } => Err(
                InstanceProfileError::InstanceProfileModificationNotSupported {
                    name_diff: name_diff.clone(),
                    path_diff: path_diff.clone(),
                },
            ),
            InstanceProfileStateDiff::InSyncExists
            | InstanceProfileStateDiff::InSyncDoesNotExist => Ok(OpCheckStatus::ExecNotRequired),
        }
    }

    async fn exec_dry(
        _op_ctx: OpCtx<'_>,
        _instance_profile_data: InstanceProfileData<'_, Id>,
        _state_current: &InstanceProfileState,
        state_desired: &InstanceProfileState,
        _diff: &InstanceProfileStateDiff,
    ) -> Result<InstanceProfileState, InstanceProfileError> {
        Ok(state_desired.clone())
    }

    async fn exec(
        _op_ctx: OpCtx<'_>,
        data: InstanceProfileData<'_, Id>,
        _state_current: &InstanceProfileState,
        state_desired: &InstanceProfileState,
        diff: &InstanceProfileStateDiff,
    ) -> Result<InstanceProfileState, InstanceProfileError> {
        match diff {
            InstanceProfileStateDiff::Added => match state_desired {
                InstanceProfileState::None => {
                    panic!(
                        "`InstanceProfileApplyOpSpec::exec` called with state_desired being None."
                    );
                }
                InstanceProfileState::Some {
                    name,
                    path,
                    instance_profile_id_and_arn: _,
                    role_associated: _,
                } => {
                    let client = data.client();
                    let create_instance_profile_output = client
                        .create_instance_profile()
                        .instance_profile_name(name)
                        .path(path)
                        .send()
                        .await
                        .map_err(|error| {
                            let instance_profile_name = name.to_string();
                            let instance_profile_path = path.to_string();

                            InstanceProfileError::InstanceProfileCreateError {
                                instance_profile_name,
                                instance_profile_path,
                                error,
                            }
                        })?;
                    let instance_profile = create_instance_profile_output
                        .instance_profile()
                        .expect("Expected instance_profile to be Some when create_instance_profile is successful.");
                    let instance_profile_id = instance_profile
                        .instance_profile_id()
                        .expect("Expected instance_profile id to be Some when create_instance_profile is successful.")
                        .to_string();
                    let instance_profile_arn = instance_profile
                        .arn()
                        .expect("Expected instance_profile ARN to be Some when create_instance_profile is successful.")
                        .to_string();
                    let instance_profile_id_and_arn =
                        InstanceProfileIdAndArn::new(instance_profile_id, instance_profile_arn);

                    Self::role_associate(client, name, path).await?;

                    let state_ensured = InstanceProfileState::Some {
                        name: name.to_string(),
                        path: path.clone(),
                        instance_profile_id_and_arn: Generated::Value(instance_profile_id_and_arn),
                        role_associated: true,
                    };

                    Ok(state_ensured)
                }
            },
            InstanceProfileStateDiff::Removed => {
                panic!(
                    "`InstanceProfileApplyOpSpec::exec` called with `InstanceProfileStateDiff::Removed`.\n\
                    An ensure should never remove an instance profile."
                );
            }
            InstanceProfileStateDiff::InSyncExists
            | InstanceProfileStateDiff::InSyncDoesNotExist => {
                unreachable!(
                    "`InstanceProfileApplyOpSpec::exec` should never be called when state is in sync."
                );
            }
            InstanceProfileStateDiff::NameOrPathModified {
                name_diff,
                path_diff,
            } => Err(InstanceProfileError::NameOrPathModificationNotSupported {
                name_diff: name_diff.clone(),
                path_diff: path_diff.clone(),
            }),
            InstanceProfileStateDiff::RoleAssociatedModified {
                role_associated_current,
                role_associated_desired: _,
            } => {
                let (name, path) = match state_desired {
                    InstanceProfileState::None => {
                        panic!(
                            "`InstanceProfileApplyOpSpec::exec` called with state_desired being None."
                        );
                    }
                    InstanceProfileState::Some {
                        name,
                        path,
                        instance_profile_id_and_arn: _,
                        role_associated: _,
                    } => (name, path),
                };

                let client = data.client();
                if *role_associated_current {
                    // Remove the association.
                    InstanceProfileCleanOpSpec::<Id>::role_disassociate(client, name, path).await?;
                } else {
                    // Associate the role.
                    Self::role_associate(client, name, path).await?;
                }
                let state_ensured = state_desired.clone();
                Ok(state_ensured)
            }
        }
    }
}
