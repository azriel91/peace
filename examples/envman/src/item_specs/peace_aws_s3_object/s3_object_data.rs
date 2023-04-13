use peace::data::{
    accessors::{ROpt, R, W},
    Data,
};

use crate::item_specs::peace_aws_s3_object::S3ObjectParams;

/// Data used to manage S3 object state.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different S3 object parameters
///   from each other.
#[derive(Data, Debug)]
pub struct S3ObjectData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    /// S3Object state parameters.
    params: W<'exec, S3ObjectParams<Id>>,
    /// IAM client to communicate with AWS.
    client: R<'exec, aws_sdk_s3::Client>,
    /// Region to use to constrain S3 object.
    region: ROpt<'exec, aws_sdk_s3::config::Region>,
}

impl<'exec, Id> S3ObjectData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    pub fn params(&self) -> &S3ObjectParams<Id> {
        &self.params
    }

    pub fn params_mut(&mut self) -> &mut S3ObjectParams<Id> {
        &mut self.params
    }

    pub fn client(&self) -> &R<'exec, aws_sdk_s3::Client> {
        &self.client
    }

    pub fn region(&self) -> &ROpt<'exec, aws_sdk_s3::config::Region> {
        &self.region
    }
}
