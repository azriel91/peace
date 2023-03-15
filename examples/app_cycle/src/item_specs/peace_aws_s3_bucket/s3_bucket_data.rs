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
pub struct S3BucketData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    /// S3Bucket state parameters.
    params: W<'op, S3BucketParams<Id>>,
    /// IAM client to communicate with AWS.
    client: R<'op, aws_sdk_s3::Client>,
    /// Region to use to constrain S3 bucket.
    region: ROpt<'op, aws_sdk_s3::Region>,
}

impl<'op, Id> S3BucketData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    pub fn params(&self) -> &S3BucketParams<Id> {
        &self.params
    }

    pub fn params_mut(&mut self) -> &mut S3BucketParams<Id> {
        &mut self.params
    }

    pub fn client(&self) -> &R<'op, aws_sdk_s3::Client> {
        &self.client
    }

    pub fn region(&self) -> &ROpt<'op, aws_sdk_s3::Region> {
        &self.region
    }
}
