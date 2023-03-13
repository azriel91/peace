use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, state::Generated, EnsureOpSpec, OpCheckStatus, OpCtx};

use crate::item_specs::peace_aws_iam_role::{
    model::RoleIdAndArn, IamRoleData, IamRoleError, IamRoleState, IamRoleStateDiff,
};

/// Ensure OpSpec for the instance profile state.
#[derive(Debug)]
pub struct IamRoleEnsureOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> EnsureOpSpec for IamRoleEnsureOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = IamRoleData<'op, Id>;
    type Error = IamRoleError;
    type State = IamRoleState;
    type StateDiff = IamRoleStateDiff;

    async fn check(
        _iam_role_data: IamRoleData<'_, Id>,
        _state_current: &IamRoleState,
        _state_desired: &IamRoleState,
        diff: &IamRoleStateDiff,
    ) -> Result<OpCheckStatus, IamRoleError> {
        match diff {
            IamRoleStateDiff::Added => {
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
            IamRoleStateDiff::Removed => {
                panic!(
                    "`IamRoleEnsureOpSpec::check` called with `IamRoleStateDiff::Removed`.\n\
                    An ensure should never remove an instance profile."
                );
            }
            IamRoleStateDiff::Modified {
                name_diff,
                path_diff,
            } => Err(IamRoleError::RoleModificationNotSupported {
                name_diff: name_diff.clone(),
                path_diff: path_diff.clone(),
            }),
            IamRoleStateDiff::InSyncExists | IamRoleStateDiff::InSyncDoesNotExist => {
                Ok(OpCheckStatus::ExecNotRequired)
            }
        }
    }

    async fn exec_dry(
        _op_ctx: OpCtx<'_>,
        _iam_role_data: IamRoleData<'_, Id>,
        _state_current: &IamRoleState,
        state_desired: &IamRoleState,
        _diff: &IamRoleStateDiff,
    ) -> Result<IamRoleState, IamRoleError> {
        Ok(state_desired.clone())
    }

    async fn exec(
        _op_ctx: OpCtx<'_>,
        data: IamRoleData<'_, Id>,
        _state_current: &IamRoleState,
        state_desired: &IamRoleState,
        diff: &IamRoleStateDiff,
    ) -> Result<IamRoleState, IamRoleError> {
        match diff {
            IamRoleStateDiff::Added => match state_desired {
                IamRoleState::None => {
                    panic!("`IamRoleEnsureOpSpec::exec` called with state_desired being None.");
                }
                IamRoleState::Some {
                    name,
                    path,
                    role_id_and_arn: _,
                } => {
                    let assume_role_policy_document =
                        include_str!("ec2_assume_role_policy_document.json");
                    let role_create_output = data
                        .client()
                        .create_role()
                        .role_name(name)
                        .path(path)
                        .assume_role_policy_document(assume_role_policy_document)
                        .send()
                        .await
                        .map_err(|error| {
                            let role_name = name.to_string();

                            IamRoleError::RoleCreateError { role_name, error }
                        })?;
                    let role = role_create_output
                        .role()
                        .expect("Expected role to be Some when created.");
                    let role_id = role
                        .role_id()
                        .expect("Expected role ID to be Some when created.");
                    let role_arn = role
                        .arn()
                        .expect("Expected role ARN to be Some when created.");

                    let state_ensured = IamRoleState::Some {
                        name: name.to_string(),
                        path: path.clone(),
                        role_id_and_arn: Generated::Value(RoleIdAndArn::new(
                            role_id.to_string(),
                            role_arn.to_string(),
                        )),
                    };

                    Ok(state_ensured)
                }
            },
            IamRoleStateDiff::Removed => {
                panic!(
                    "`IamRoleEnsureOpSpec::exec` called with `IamRoleStateDiff::Removed`.\n\
                    An ensure should never remove an instance profile."
                );
            }
            IamRoleStateDiff::InSyncExists | IamRoleStateDiff::InSyncDoesNotExist => {
                unreachable!(
                    "`IamRoleEnsureOpSpec::exec` should never be called when state is in sync."
                );
            }
            IamRoleStateDiff::Modified { .. } => {
                panic!("Name or path modification is not supported.");
            }
        }
    }
}
