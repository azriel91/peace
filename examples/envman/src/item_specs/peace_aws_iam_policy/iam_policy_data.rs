use peace::data::{
    accessors::{WOpt, R},
    Data,
};

use crate::item_specs::peace_aws_iam_policy::model::ManagedPolicyArn;

/// Data used to manage instance profile state.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different instance profile
///   parameters from each other.
#[derive(Data, Debug)]
pub struct IamPolicyData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    /// Workaround for managed policy ARN param.
    ///
    /// Hack: Remove this when referential param values is implemented.
    managed_policy_arn: WOpt<'exec, ManagedPolicyArn<Id>>,
    /// IAM client to communicate with AWS.
    client: R<'exec, aws_sdk_iam::Client>,
}

impl<'exec, Id> IamPolicyData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    pub fn client(&self) -> &R<'exec, aws_sdk_iam::Client> {
        &self.client
    }

    /// Hack: Remove this when referential param values is implemented.
    pub fn managed_policy_arn(&self) -> Option<&ManagedPolicyArn<Id>> {
        self.managed_policy_arn.as_ref()
    }

    /// Hack: Remove this when referential param values is implemented.
    pub fn managed_policy_arn_mut(&mut self) -> &mut WOpt<'exec, ManagedPolicyArn<Id>> {
        &mut self.managed_policy_arn
    }
}
