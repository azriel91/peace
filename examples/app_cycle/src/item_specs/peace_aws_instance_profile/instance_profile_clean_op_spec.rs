use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, state::Generated, CleanOpSpec, OpCheckStatus};

use crate::item_specs::peace_aws_instance_profile::{
    InstanceProfileData, InstanceProfileError, InstanceProfileState,
};

/// `CleanOpSpec` for the instance profile state.
#[derive(Debug, Default)]
pub struct InstanceProfileCleanOpSpec<Id>(PhantomData<Id>);

impl<Id> InstanceProfileCleanOpSpec<Id> {
    pub(crate) async fn role_disassociate(
        client: &aws_sdk_iam::Client,
        name: &str,
        path: &str,
    ) -> Result<(), InstanceProfileError> {
        client
            .remove_role_from_instance_profile()
            .instance_profile_name(name)
            .role_name(name)
            .send()
            .await
            .map_err(|error| {
                let instance_profile_name = name.to_string();
                let instance_profile_path = path.to_string();

                InstanceProfileError::InstanceProfileRoleRemoveError {
                    instance_profile_name,
                    instance_profile_path,
                    error,
                }
            })?;
        Ok(())
    }
}

#[async_trait(?Send)]
impl<Id> CleanOpSpec for InstanceProfileCleanOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = InstanceProfileData<'op, Id>;
    type Error = InstanceProfileError;
    type State = InstanceProfileState;

    async fn check(
        _data: InstanceProfileData<'_, Id>,
        state_current: &InstanceProfileState,
    ) -> Result<OpCheckStatus, InstanceProfileError> {
        let op_check_status = match state_current {
            InstanceProfileState::None => OpCheckStatus::ExecNotRequired,
            InstanceProfileState::Some {
                name: _,
                path: _,
                instance_profile_id_and_arn,
                role_associated,
            } => {
                let mut steps_required = 0;
                if *role_associated {
                    steps_required += 1;
                }
                if matches!(instance_profile_id_and_arn, Generated::Value(_)) {
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
        _data: InstanceProfileData<'_, Id>,
        _state_current: &InstanceProfileState,
    ) -> Result<(), InstanceProfileError> {
        Ok(())
    }

    async fn exec(
        data: InstanceProfileData<'_, Id>,
        state_current: &InstanceProfileState,
    ) -> Result<(), InstanceProfileError> {
        match state_current {
            InstanceProfileState::None => {}
            InstanceProfileState::Some {
                name,
                path,
                instance_profile_id_and_arn,
                role_associated,
            } => {
                let client = data.client();
                if *role_associated {
                    Self::role_disassociate(client, name, path).await?;
                }
                if let Generated::Value(instance_profile_id_and_arn) = instance_profile_id_and_arn {
                    client
                        .delete_instance_profile()
                        .instance_profile_name(name)
                        .send()
                        .await
                        .map_err(|error| {
                            let instance_profile_name = name.to_string();
                            let instance_profile_path = path.to_string();
                            let instance_profile_id = instance_profile_id_and_arn.id().to_string();
                            let instance_profile_arn =
                                instance_profile_id_and_arn.arn().to_string();

                            InstanceProfileError::InstanceProfileDeleteError {
                                instance_profile_name,
                                instance_profile_path,
                                instance_profile_id,
                                instance_profile_arn,
                                error,
                            }
                        })?;
                }
            }
        };

        Ok(())
    }
}
