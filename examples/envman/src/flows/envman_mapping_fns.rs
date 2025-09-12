use peace::{
    enum_iterator::Sequence,
    params::{FromFunc, MappingFn, MappingFnId, MappingFnImpl, MappingFns},
    profile_model::Profile,
};
use serde::{Deserialize, Serialize};

use crate::items::{peace_aws_iam_policy::IamPolicyState, peace_aws_s3_bucket::S3BucketState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Sequence)]
#[allow(non_camel_case_types)]
#[enum_iterator(crate = peace::enum_iterator)]
pub enum EnvmanMappingFns {
    /// Returns the `IamRole` name from profile.
    IamRoleNameFromProfile_v0_1_0,
    /// Returns the `IamRole` Managed Policy ARN from the `IamPolicyState`'s
    /// `policy_id_arn_version`.
    IamRoleManagedPolicyArnFromIamPolicyState_v0_1_0,
    /// Returns the `S3Bucket` name from the `S3BucketState`.
    S3BucketNameFromS3BucketState_v0_1_0,
}

impl MappingFns for EnvmanMappingFns {
    fn id(self) -> MappingFnId {
        let name = match self {
            Self::IamRoleNameFromProfile_v0_1_0 => "IamRoleNameFromProfile_v0_1_0",
            Self::IamRoleManagedPolicyArnFromIamPolicyState_v0_1_0 => {
                "IamRoleManagedPolicyArnFromIamPolicyState_v0_1_0"
            }
            Self::S3BucketNameFromS3BucketState_v0_1_0 => "S3BucketNameFromS3BucketState_v0_1_0",
        };
        MappingFnId::new(name.to_string())
    }

    fn mapping_fn(self) -> Box<dyn MappingFn> {
        match self {
            Self::IamRoleNameFromProfile_v0_1_0 => {
                MappingFnImpl::from_func(|profile: &Profile| Some(profile.to_string()))
            }
            Self::IamRoleManagedPolicyArnFromIamPolicyState_v0_1_0 => {
                MappingFnImpl::from_func(IamPolicyState::policy_id_arn_version)
            }
            Self::S3BucketNameFromS3BucketState_v0_1_0 => {
                MappingFnImpl::from_func(S3BucketState::bucket_name)
            }
        }
    }
}
