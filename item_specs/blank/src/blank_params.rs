use std::marker::PhantomData;

use derivative::Derivative;
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
#[derive(Clone, Derivative, PartialEq, Eq, Deserialize, Serialize)]
#[derivative(Debug)]
pub struct BlankParams<Id> {
    /// Source / desired value for the state.
    src: BlankSrc,
    /// Destination / current value of the state.
    dest: BlankDest,
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

    /// Returns the source / desired value for the state.
    pub fn src(&self) -> &BlankSrc {
        &self.src
    }

    /// Returns a mutable reference to the source / desired value for the state.
    pub fn src_mut(&mut self) -> &mut BlankSrc {
        &mut self.src
    }

    /// Returns the destination / current value of the state.
    pub fn dest(&self) -> &BlankDest {
        &self.dest
    }

    /// Returns a mutable reference to the destination / current value of the
    /// state.
    pub fn dest_mut(&mut self) -> &mut BlankDest {
        &mut self.dest
    }
}
