use std::marker::PhantomData;

use derivative::Derivative;
use peace::params::ValueSpec;
use serde::{Deserialize, Serialize};

use crate::{BlankDest, BlankSrc};

/// Blank item parameters.
///
/// The `Id` type parameter is needed for each blank params to be a distinct
/// type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different blank parameters
///   from each other.
#[derive(Derivative, ValueSpec, PartialEq, Eq, Deserialize, Serialize)]
#[derivative(Clone, Debug)]
#[serde(bound = "")]
pub struct BlankParams<Id> {
    /// Source / desired value for the state.
    pub src: BlankSrc,
    /// Destination / current value of the state.
    pub dest: BlankDest,
    /// Marker for unique blank parameters type.
    marker: PhantomData<Id>,
}

impl<Id> BlankParams<Id> {
    /// Returns new `BlankParams`.
    pub fn new(src: BlankSrc, dest: BlankDest) -> Self {
        Self {
            src,
            dest,
            marker: PhantomData,
        }
    }
}
