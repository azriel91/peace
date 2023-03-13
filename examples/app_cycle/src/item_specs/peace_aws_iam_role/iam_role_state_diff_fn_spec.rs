use peace::cfg::{async_trait, StateDiffFnSpec};

use crate::item_specs::peace_aws_iam_role::{IamRoleError, IamRoleState, IamRoleStateDiff};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct IamRoleStateDiffFnSpec;

#[async_trait(?Send)]
impl StateDiffFnSpec for IamRoleStateDiffFnSpec {
    type Data<'op> = &'op ();
    type Error = IamRoleError;
    type State = IamRoleState;
    type StateDiff = IamRoleStateDiff;

    async fn exec(
        _: &(),
        state_current: &IamRoleState,
        state_desired: &IamRoleState,
    ) -> Result<Self::StateDiff, IamRoleError> {
        let diff = match (state_current, state_desired) {
            (IamRoleState::None, IamRoleState::None) => IamRoleStateDiff::InSyncDoesNotExist,
            (IamRoleState::None, IamRoleState::Some { .. }) => IamRoleStateDiff::Added,
            (IamRoleState::Some { .. }, IamRoleState::None) => IamRoleStateDiff::Removed,
            (
                IamRoleState::Some {
                    name: name_current,
                    path: path_current,
                    role_id_and_arn: _,
                    managed_policy_attachment: managed_policy_attachment_current,
                },
                IamRoleState::Some {
                    name: name_desired,
                    path: path_desired,
                    role_id_and_arn: _,
                    managed_policy_attachment: managed_policy_attachment_desired,
                },
            ) => {
                let name_diff = if name_current != name_desired {
                    Some((name_current.clone(), name_desired.clone()))
                } else {
                    None
                };

                let path_diff = if path_current != path_desired {
                    Some((path_current.clone(), path_desired.clone()))
                } else {
                    None
                };

                if name_diff.is_none() && path_diff.is_none() {
                    if managed_policy_attachment_current != managed_policy_attachment_desired {
                        IamRoleStateDiff::ManagedPolicyAttachmentModified {
                            managed_policy_attachment_current: managed_policy_attachment_current
                                .clone(),
                            managed_policy_attachment_desired: managed_policy_attachment_desired
                                .clone(),
                        }
                    } else {
                        IamRoleStateDiff::InSyncExists
                    }
                } else {
                    IamRoleStateDiff::NameOrPathModified {
                        name_diff,
                        path_diff,
                    }
                }
            }
        };

        Ok(diff)
    }
}
