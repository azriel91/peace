use crate::items::peace_aws_instance_profile::{
    InstanceProfileError, InstanceProfileState, InstanceProfileStateDiff,
};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct InstanceProfileStateDiffFn;

impl InstanceProfileStateDiffFn {
    pub async fn state_diff(
        state_current: &InstanceProfileState,
        state_goal: &InstanceProfileState,
    ) -> Result<InstanceProfileStateDiff, InstanceProfileError> {
        let diff = match (state_current, state_goal) {
            (InstanceProfileState::None, InstanceProfileState::None) => {
                InstanceProfileStateDiff::InSyncDoesNotExist
            }
            (InstanceProfileState::None, InstanceProfileState::Some { .. }) => {
                InstanceProfileStateDiff::Added
            }
            (InstanceProfileState::Some { .. }, InstanceProfileState::None) => {
                InstanceProfileStateDiff::Removed
            }
            (
                InstanceProfileState::Some {
                    name: name_current,
                    path: path_current,
                    instance_profile_id_and_arn: _,
                    role_associated: role_associated_current,
                },
                InstanceProfileState::Some {
                    name: name_goal,
                    path: path_goal,
                    instance_profile_id_and_arn: _,
                    role_associated: role_associated_goal,
                },
            ) => {
                let role_associated_current = *role_associated_current;
                let role_associated_goal = *role_associated_goal;

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
                    if role_associated_current == role_associated_goal {
                        InstanceProfileStateDiff::InSyncExists
                    } else {
                        InstanceProfileStateDiff::RoleAssociatedModified {
                            role_associated_current,
                            role_associated_goal,
                        }
                    }
                } else {
                    InstanceProfileStateDiff::NameOrPathModified {
                        name_diff,
                        path_diff,
                    }
                }
            }
        };

        Ok(diff)
    }
}
