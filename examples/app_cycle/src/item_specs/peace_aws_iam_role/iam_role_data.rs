use peace::{
    cfg::ROpt,
    data::{Data, R, W},
};

use crate::item_specs::{
    peace_aws_iam_policy::model::ManagedPolicyArn, peace_aws_iam_role::IamRoleParams,
};

/// Data used to manage instance profile state.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different instance profile
///   parameters from each other.
#[derive(Data, Debug)]
pub struct IamRoleData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    /// IamRole state parameters.
    params: W<'op, IamRoleParams<Id>>,
    /// Workaround for managed policy ARN param.
    ///
    /// Hack: Remove this when referential param values is implemented.
    managed_policy_arn: ROpt<'op, ManagedPolicyArn<Id>>,
    /// IAM client to communicate with AWS.
    client: R<'op, aws_sdk_iam::Client>,
}

impl<'op, Id> IamRoleData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    pub fn params(&self) -> &IamRoleParams<Id> {
        &self.params
    }

    pub fn params_mut(&mut self) -> &mut IamRoleParams<Id> {
        &mut self.params
    }

    /// Hack: Remove this when referential param values is implemented.
    pub fn managed_policy_arn(&self) -> Option<&ManagedPolicyArn<Id>> {
        self.managed_policy_arn.as_ref()
    }

    pub fn client(&self) -> &R<'op, aws_sdk_iam::Client> {
        &self.client
    }
}
