use peace::{
    cfg::WOpt,
    data::{Data, R, W},
};

use crate::item_specs::peace_aws_iam_policy::{model::ManagedPolicyArn, IamPolicyParams};

/// Data used to manage instance profile state.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different instance profile
///   parameters from each other.
#[derive(Data, Debug)]
pub struct IamPolicyData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    /// IamPolicy state parameters.
    params: W<'op, IamPolicyParams<Id>>,
    /// Workaround for managed policy ARN param.
    ///
    /// Hack: Remove this when referential param values is implemented.
    managed_policy_arn: WOpt<'op, ManagedPolicyArn<Id>>,
    /// IAM client to communicate with AWS.
    client: R<'op, aws_sdk_iam::Client>,
}

impl<'op, Id> IamPolicyData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    pub fn params(&self) -> &IamPolicyParams<Id> {
        &self.params
    }

    pub fn params_mut(&mut self) -> &mut IamPolicyParams<Id> {
        &mut self.params
    }

    pub fn client(&self) -> &R<'op, aws_sdk_iam::Client> {
        &self.client
    }

    /// Hack: Remove this when referential param values is implemented.
    pub fn managed_policy_arn(&self) -> Option<&ManagedPolicyArn<Id>> {
        self.managed_policy_arn.as_ref()
    }

    /// Hack: Remove this when referential param values is implemented.
    pub fn managed_policy_arn_mut(&mut self) -> &mut WOpt<'op, ManagedPolicyArn<Id>> {
        &mut self.managed_policy_arn
    }
}
