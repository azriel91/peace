use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, state::Generated, CleanOpSpec, OpCheckStatus};

use crate::item_specs::peace_aws_iam_role::{IamRoleData, IamRoleError, IamRoleState};

/// `CleanOpSpec` for the instance profile state.
#[derive(Debug, Default)]
pub struct IamRoleCleanOpSpec<Id>(PhantomData<Id>);

impl<Id> IamRoleCleanOpSpec<Id> {
    pub(crate) async fn managed_policy_detach(
        client: &aws_sdk_iam::Client,
        name: &str,
        path: &str,
        managed_policy_arn: &str,
    ) -> Result<(), IamRoleError> {
        client
            .detach_role_policy()
            .role_name(name)
            .policy_arn(managed_policy_arn)
            .send()
            .await
            .map_err(|error| {
                let role_name = name.to_string();
                let role_path = path.to_string();

                IamRoleError::ManagedPolicyDetachError {
                    role_name,
                    role_path,
                    error,
                }
            })?;
        Ok(())
    }
}

#[async_trait(?Send)]
impl<Id> CleanOpSpec for IamRoleCleanOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = IamRoleData<'op, Id>;
    type Error = IamRoleError;
    type State = IamRoleState;

    async fn check(
        _data: IamRoleData<'_, Id>,
        state_current: &IamRoleState,
    ) -> Result<OpCheckStatus, IamRoleError> {
        let op_check_status = match state_current {
            IamRoleState::None => OpCheckStatus::ExecNotRequired,
            IamRoleState::Some {
                name: _,
                path: _,
                role_id_and_arn,
                managed_policy_attachment,
            } => {
                let mut steps_required = 0;
                if managed_policy_attachment.attached() {
                    steps_required += 1;
                }
                if matches!(role_id_and_arn, Generated::Value(_)) {
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
        _data: IamRoleData<'_, Id>,
        _state_current: &IamRoleState,
    ) -> Result<(), IamRoleError> {
        Ok(())
    }

    async fn exec(
        data: IamRoleData<'_, Id>,
        state_current: &IamRoleState,
    ) -> Result<(), IamRoleError> {
        match state_current {
            IamRoleState::None => {}
            IamRoleState::Some {
                name,
                path,
                role_id_and_arn,
                managed_policy_attachment,
            } => {
                let client = data.client();
                if managed_policy_attachment.attached() {
                    Self::managed_policy_detach(
                        client,
                        name,
                        path,
                        managed_policy_attachment.arn(),
                    )
                    .await?;
                }
                if let Generated::Value(role_id_and_arn) = role_id_and_arn {
                    client
                        .delete_role()
                        .role_name(name)
                        .send()
                        .await
                        .map_err(|error| {
                            let role_name = name.to_string();
                            let role_id = role_id_and_arn.id().to_string();
                            let role_arn = role_id_and_arn.arn().to_string();

                            IamRoleError::RoleDeleteError {
                                role_name,
                                role_id,
                                role_arn,
                                error,
                            }
                        })?;
                }
            }
        };

        Ok(())
    }
}
