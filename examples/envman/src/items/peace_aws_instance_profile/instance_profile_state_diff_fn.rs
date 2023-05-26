use crate::items::peace_aws_instance_profile::{
    InstanceProfileError, InstanceProfileState, InstanceProfileStateDiff,
};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct InstanceProfileStateDiffFn;

impl InstanceProfileStateDiffFn {
    pub async fn state_diff(
        state_current: &InstanceProfileState,
        state_desired: &InstanceProfileState,
    ) -> Result<InstanceProfileStateDiff, InstanceProfileError> {
        let diff = match (state_current, state_desired) {
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
                    name: name_desired,
                    path: path_desired,
                    instance_profile_id_and_arn: _,
                    role_associated: role_associated_desired,
                },
            ) => {
                let role_associated_current = *role_associated_current;
                let role_associated_desired = *role_associated_desired;

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
                    if role_associated_current == role_associated_desired {
                        InstanceProfileStateDiff::InSyncExists
                    } else {
                        InstanceProfileStateDiff::RoleAssociatedModified {
                            role_associated_current,
                            role_associated_desired,
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
