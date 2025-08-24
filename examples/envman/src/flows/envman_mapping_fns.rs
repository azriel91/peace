use peace::{
    params::{FromFunc, MappingFn, MappingFnImpl, MappingFns},
    profile_model::Profile,
};
use serde::{Deserialize, Serialize};

use crate::items::peace_aws_iam_policy::IamPolicyState;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum EnvmanMappingFns {
    /// Sets the `IamRole` name from profile.
    IamRoleNameFromProfile,
    /// Sets the `IamRole` Managed Policy ARN from the `IamPolicyState`'s
    /// `policy_id_arn_version`.
    IamRoleManagedPolicyArnFromIamPolicyState,
}

impl MappingFns for EnvmanMappingFns {
    fn iter() -> impl Iterator<Item = Self> + ExactSizeIterator {
        [
            Self::IamRoleManagedPolicyArnFromIamPolicyState,
            Self::IamRoleNameFromProfile,
        ]
        .into_iter()
    }

    fn mapping_fn(self) -> Box<dyn MappingFn> {
        match self {
            Self::IamRoleNameFromProfile => Box::new(MappingFnImpl::from_func(
                Some(String::from("name")),
                |profile: &Profile| Some(profile.to_string()),
            )),
            Self::IamRoleManagedPolicyArnFromIamPolicyState => Box::new(MappingFnImpl::from_func(
                Some(String::from("managed_policy_arn")),
                IamPolicyState::policy_id_arn_version,
            )),
        }
    }
}
