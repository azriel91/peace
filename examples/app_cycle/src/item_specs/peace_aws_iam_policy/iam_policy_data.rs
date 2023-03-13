use peace::data::{Data, R, W};

use crate::item_specs::peace_aws_iam_policy::IamPolicyParams;

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
}
