use std::marker::PhantomData;

use peace::data::{accessors::R, Data};

/// Data used to manage instance profile state.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different instance profile
///   parameters from each other.
#[derive(Data, Debug)]
pub struct IamRoleData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    /// IAM client to communicate with AWS.
    client: R<'exec, aws_sdk_iam::Client>,

    /// Marker.
    marker: PhantomData<Id>,
}

impl<'exec, Id> IamRoleData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    pub fn client(&self) -> &R<'exec, aws_sdk_iam::Client> {
        &self.client
    }
}
