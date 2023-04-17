use std::marker::PhantomData;

use peace::data::{
    accessors::{ROpt, R},
    Data,
};

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
    /// IAM client to communicate with AWS.
    client: R<'exec, aws_sdk_s3::Client>,
    /// Region to use to constrain S3 bucket.
    region: ROpt<'exec, aws_sdk_s3::config::Region>,
    /// Marker.
    marker: PhantomData<Id>,
}

impl<'exec, Id> S3BucketData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    pub fn client(&self) -> &R<'exec, aws_sdk_s3::Client> {
        &self.client
    }

    pub fn region(&self) -> &ROpt<'exec, aws_sdk_s3::config::Region> {
        &self.region
    }
}
