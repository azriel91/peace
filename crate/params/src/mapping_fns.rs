use std::{fmt::Debug, hash::Hash};

use enum_iterator::Sequence;
use serde::{de::DeserializeOwned, Serialize};

use crate::{MappingFn, MappingFnId, MappingFnImpl};

/// Enum to give versioned IDs to mapping functions, so that params specs and
/// value specs can be serialized.
///
/// Item parameters may be mapped from other items' state, and that logic
/// exists as code. However, we want the ability to store (remember) those
/// mappings across command executions. If a closure is held in the params
/// specs and value specs, then they cannot be serialized. However, if we
/// place that logic elsewhere (like in the `CmdCtxTypes` implementation),
/// and have an intermediate enum to represent the mapping functions, we can
/// serialize the enum instead of the closure.
///
/// # Examples
///
/// ```rust,ignore
/// use peace::{
///     enum_iterator::Sequence,
///     params::{FromFunc, MappingFn, MappingFnId, MappingFnImpl, MappingFns},
///     profile_model::Profile,
/// };
/// use serde::{Deserialize, Serialize};
///
/// use crate::items::{peace_aws_iam_policy::IamPolicyState, peace_aws_s3_bucket::S3BucketState};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Sequence)]
/// #[allow(non_camel_case_types)]
/// #[enum_iterator(crate = peace::enum_iterator)]
/// pub enum EnvmanMappingFns {
///     /// Returns the `IamRole` name from profile.
///     IamRoleNameFromProfile_v0_1_0,
///     /// Returns the `IamRole` Managed Policy ARN from the `IamPolicyState`'s
///     /// `policy_id_arn_version`.
///     IamRoleManagedPolicyArnFromIamPolicyState_v0_1_0,
///     /// Returns the `S3Bucket` name from the `S3BucketState`.
///     S3BucketNameFromS3BucketState_v0_1_0,
/// }
///
/// impl MappingFns for EnvmanMappingFns {
///     fn id(self) -> MappingFnId {
///         let name = match self {
///             Self::IamRoleNameFromProfile_v0_1_0 => "IamRoleNameFromProfile_v0_1_0",
///             Self::IamRoleManagedPolicyArnFromIamPolicyState_v0_1_0 => {
///                 "IamRoleManagedPolicyArnFromIamPolicyState_v0_1_0"
///             }
///             Self::S3BucketNameFromS3BucketState_v0_1_0 => {
///                 "S3BucketNameFromS3BucketState_v0_1_0"
///             }
///         };
///         MappingFnId::new(name.to_string())
///     }
///
///     fn mapping_fn(self) -> Box<dyn MappingFn> {
///         match self {
///             Self::IamRoleNameFromProfile_v0_1_0 => {
///                 MappingFnImpl::from_func(|profile: &Profile| Some(profile.to_string()))
///             }
///             Self::IamRoleManagedPolicyArnFromIamPolicyState_v0_1_0 => {
///                 MappingFnImpl::from_func(IamPolicyState::policy_id_arn_version)
///             }
///             Self::S3BucketNameFromS3BucketState_v0_1_0 => {
///                 MappingFnImpl::from_func(S3BucketState::bucket_name)
///             }
///         }
///     }
/// }
/// ```
pub trait MappingFns:
    Clone
    + Copy
    + Debug
    + Hash
    + PartialEq
    + Eq
    + Serialize
    + DeserializeOwned
    + Sequence
    + Send
    + Sync
    + 'static
{
    /// Returns a string representation of the mapping function name.
    ///
    /// # Implementors
    ///
    /// The returned ID is considered API, and should be stable. This means
    /// you should name each variant with a version number, and never remove
    /// that variant, e.g. `MappingFnId::new("ServerNameFromProfile_V1_0_0"
    /// )`.
    ///
    /// That way, previously stored mapping function IDs can still be
    /// deserialized, and tool developers can opt-in to upgrading to the newer
    /// mapping functions when ready.
    fn id(self) -> MappingFnId;

    /// Returns the mapping function corresponding to the given variant.
    fn mapping_fn(self) -> Box<dyn MappingFn>;
}

impl MappingFns for () {
    fn id(self) -> MappingFnId {
        MappingFnId::new(String::from(""))
    }

    fn mapping_fn(self) -> Box<dyn MappingFn> {
        Box::new(MappingFnImpl::<(), _, ()>::empty())
    }
}
