use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, state::Generated, CleanOpSpec, OpCheckStatus};

use crate::item_specs::peace_aws_iam_role::{IamRoleData, IamRoleError, IamRoleState};

/// `CleanOpSpec` for the instance profile state.
#[derive(Debug, Default)]
pub struct IamRoleCleanOpSpec<Id>(PhantomData<Id>);

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
            } => {
                let mut steps_required = 0;
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
                path: _,
                role_id_and_arn,
            } => {
                if let Generated::Value(role_id_and_arn) = role_id_and_arn {
                    data.client()
                        .delete_role()
                        .role_name(role_id_and_arn.id())
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
