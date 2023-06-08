use crate::items::peace_aws_iam_role::{IamRoleError, IamRoleState, IamRoleStateDiff};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct IamRoleStateDiffFn;

impl IamRoleStateDiffFn {
    pub async fn state_diff(
        state_current: &IamRoleState,
        state_goal: &IamRoleState,
    ) -> Result<IamRoleStateDiff, IamRoleError> {
        let diff = match (state_current, state_goal) {
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
                    name: name_goal,
                    path: path_goal,
                    role_id_and_arn: _,
                    managed_policy_attachment: managed_policy_attachment_goal,
                },
            ) => {
                let name_diff = if name_current != name_goal {
                    Some((name_current.clone(), name_goal.clone()))
                } else {
                    None
                };

                let path_diff = if path_current != path_goal {
                    Some((path_current.clone(), path_goal.clone()))
                } else {
                    None
                };

                if name_diff.is_none() && path_diff.is_none() {
                    if managed_policy_attachment_current != managed_policy_attachment_goal {
                        IamRoleStateDiff::ManagedPolicyAttachmentModified {
                            managed_policy_attachment_current: managed_policy_attachment_current
                                .clone(),
                            managed_policy_attachment_goal: managed_policy_attachment_goal.clone(),
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
