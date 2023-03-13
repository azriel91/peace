use peace::data::{Data, R, W};

use crate::item_specs::peace_aws_iam_role::IamRoleParams;

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

    pub fn client(&self) -> &R<'op, aws_sdk_iam::Client> {
        &self.client
    }
}
