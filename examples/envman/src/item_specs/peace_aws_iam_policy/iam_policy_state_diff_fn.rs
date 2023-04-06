use crate::item_specs::peace_aws_iam_policy::{IamPolicyError, IamPolicyState, IamPolicyStateDiff};

/// Tar extraction status diff function.
#[derive(Debug)]
pub struct IamPolicyStateDiffFn;

impl IamPolicyStateDiffFn {
    pub async fn state_diff(
        state_current: &IamPolicyState,
        state_desired: &IamPolicyState,
    ) -> Result<IamPolicyStateDiff, IamPolicyError> {
        let diff = match (state_current, state_desired) {
            (IamPolicyState::None, IamPolicyState::None) => IamPolicyStateDiff::InSyncDoesNotExist,
            (IamPolicyState::None, IamPolicyState::Some { .. }) => IamPolicyStateDiff::Added,
            (IamPolicyState::Some { .. }, IamPolicyState::None) => IamPolicyStateDiff::Removed,
            (
                IamPolicyState::Some {
                    name: name_current,
                    path: path_current,
                    policy_document: document_current,
                    policy_id_arn_version: _,
                },
                IamPolicyState::Some {
                    name: name_desired,
                    path: path_desired,
                    policy_document: document_desired,
                    policy_id_arn_version: _,
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
                    if document_current == document_desired {
                        IamPolicyStateDiff::InSyncExists
                    } else {
                        IamPolicyStateDiff::DocumentModified {
                            document_current: document_current.clone(),
                            document_desired: document_desired.clone(),
                        }
                    }
                } else {
                    IamPolicyStateDiff::NameOrPathModified {
                        name_diff,
                        path_diff,
                    }
                }
            }
        };

        Ok(diff)
    }
}
