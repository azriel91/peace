use peace::data::{
    accessors::{ROpt, R, W},
    Data,
};

use crate::item_specs::peace_aws_s3_bucket::S3BucketParams;

/// Data used to manage S3 bucket state.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different S3 bucket parameters
///   from each other.
#[derive(Data, Debug)]
pub struct S3BucketData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    /// S3Bucket state parameters.
    params: W<'exec, S3BucketParams<Id>>,
    /// IAM client to communicate with AWS.
    client: R<'exec, aws_sdk_s3::Client>,
    /// Region to use to constrain S3 bucket.
    region: ROpt<'exec, aws_sdk_s3::config::Region>,
}

impl<'exec, Id> S3BucketData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    pub fn params(&self) -> &S3BucketParams<Id> {
        &self.params
    }

    pub fn params_mut(&mut self) -> &mut S3BucketParams<Id> {
        &mut self.params
    }

    pub fn client(&self) -> &R<'exec, aws_sdk_s3::Client> {
        &self.client
    }

    pub fn region(&self) -> &ROpt<'exec, aws_sdk_s3::config::Region> {
        &self.region
    }
}
