use peace::data::{Data, R, W};

use crate::item_specs::peace_aws_instance_profile::InstanceProfileParams;

/// Data used to manage instance profile state.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different instance profile
///   parameters from each other.
#[derive(Data, Debug)]
pub struct InstanceProfileData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    /// InstanceProfile state parameters.
    params: W<'op, InstanceProfileParams<Id>>,
    /// IAM client to communicate with AWS.
    client: R<'op, aws_sdk_iam::Client>,
}

impl<'op, Id> InstanceProfileData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    pub fn params(&self) -> &InstanceProfileParams<Id> {
        &self.params
    }

    pub fn params_mut(&mut self) -> &mut InstanceProfileParams<Id> {
        &mut self.params
    }

    pub fn client(&self) -> &R<'op, aws_sdk_iam::Client> {
        &self.client
    }
}
