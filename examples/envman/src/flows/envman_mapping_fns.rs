use peace::{
    params::{FromFunc, MappingFn, MappingFnImpl, MappingFnName, MappingFns},
    profile_model::Profile,
};
use serde::{Deserialize, Serialize};

use crate::items::{peace_aws_iam_policy::IamPolicyState, peace_aws_s3_bucket::S3BucketState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum EnvmanMappingFns {
    /// Sets the `IamRole` name from profile.
    IamRoleNameFromProfile,
    /// Sets the `IamRole` Managed Policy ARN from the `IamPolicyState`'s
    /// `policy_id_arn_version`.
    IamRoleManagedPolicyArnFromIamPolicyState,
    /// Returns the `S3Bucket` name from the `S3BucketState`.
    S3BucketNameFromS3BucketState,
}

impl MappingFns for EnvmanMappingFns {
    fn iter() -> impl Iterator<Item = Self> + ExactSizeIterator {
        [
            Self::IamRoleManagedPolicyArnFromIamPolicyState,
            Self::IamRoleNameFromProfile,
            Self::S3BucketNameFromS3BucketState,
        ]
        .into_iter()
    }

    fn name(self) -> MappingFnName {
        let name = match self {
            Self::IamRoleNameFromProfile => "IamRoleNameFromProfile",
            Self::IamRoleManagedPolicyArnFromIamPolicyState => {
                "IamRoleManagedPolicyArnFromIamPolicyState"
            }
            Self::S3BucketNameFromS3BucketState => "S3BucketNameFromS3BucketState",
        };
        MappingFnName::new(name.to_string())
    }

    fn mapping_fn(self) -> Box<dyn MappingFn> {
        match self {
            Self::IamRoleNameFromProfile => {
                MappingFnImpl::from_func(|profile: &Profile| Some(profile.to_string()))
            }
            Self::IamRoleManagedPolicyArnFromIamPolicyState => {
                MappingFnImpl::from_func(IamPolicyState::policy_id_arn_version)
            }
            Self::S3BucketNameFromS3BucketState => {
                MappingFnImpl::from_func(S3BucketState::bucket_name)
            }
        }
    }
}
