use std::{fmt::Debug, hash::Hash};

use serde::{de::DeserializeOwned, Serialize};

use crate::{MappingFn, MappingFnName};

/// Enum to give names to mapping functions, so that params specs and value
/// specs can be serialized.
///
/// Item parameters may be mapped from other items' state, and that logic
/// exists as code. However, we want the ability to store (remember) those
/// mappings across command executions. If a closure is held in the params
/// specs and value specs, then they cannot be serialized. However, if we
/// place that logic elsewhere (like in the `CmdCtxTypes` implementation),
/// and have an intermediate enum to represent the mapping functions, we can
/// serialize the enum instead of the closure.
pub trait MappingFns:
    Clone + Copy + Debug + Hash + PartialEq + Eq + Serialize + DeserializeOwned + Send + Sync + 'static
{
    /// Returns an iterator over all variants of these mapping functions.
    fn iter() -> impl Iterator<Item = Self> + ExactSizeIterator;

    /// Returns a string representation of the mapping function name.
    ///
    /// # Implementors
    ///
    /// The returned name is considered API, and should be stable.
    fn name(&self) -> MappingFnName;

    /// Returns the mapping function corresponding to the given variant.
    fn mapping_fn(self) -> Box<dyn MappingFn>;
}

impl MappingFns for () {
    fn iter() -> impl Iterator<Item = Self> + ExactSizeIterator {
        std::iter::empty()
    }

    fn name(&self) -> MappingFnName {
        unreachable!("`()` is not intended to be used as a mapping function name, but an indicator that no mapping functions are used.")
    }

    fn mapping_fn(self) -> Box<dyn MappingFn> {
        unreachable!("`()` is not intended to be used as a mapping function name, but an indicator that no mapping functions are used.")
    }
}
