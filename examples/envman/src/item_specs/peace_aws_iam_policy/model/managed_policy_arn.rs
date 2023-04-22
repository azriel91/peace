use std::marker::PhantomData;

use derivative::Derivative;
use serde::{Deserialize, Serialize};

/// A managed policy ARN.
///
/// Hack: Remove this when referential param values is implemented.
#[derive(Derivative, PartialEq, Eq, Deserialize, Serialize)]
#[derivative(Clone, Debug)]
pub struct ManagedPolicyArn<Id> {
    /// ARN of the managed policy.
    arn: String,
    /// Marker.
    marker: PhantomData<Id>,
}

impl<Id> ManagedPolicyArn<Id> {
    pub fn new(arn: String) -> Self {
        Self {
            arn,
            marker: PhantomData,
        }
    }

    pub fn arn(&self) -> &str {
        self.arn.as_ref()
    }
}

impl<Id> std::ops::Deref for ManagedPolicyArn<Id> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.arn
    }
}
