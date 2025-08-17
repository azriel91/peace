use std::{fmt::Debug, hash::Hash};

use serde::{de::DeserializeOwned, Serialize};

use crate::{MappingFn, ParamsResolveError, ValueResolutionCtx};

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

    /// Returns the mapping function corresponding to the given variant.
    fn mapping_fn(self) -> Box<dyn MappingFn>;

    /// Returns an error indicating that the mapping function could not be
    /// resolved.
    fn into_params_resolve_error(
        &self,
        value_resolution_ctx: ValueResolutionCtx,
    ) -> ParamsResolveError {
        ParamsResolveError::MappingFnResolve {
            value_resolution_ctx,
            mapping_fn: serde_yaml::to_string(self).unwrap_or_else(|_| format!("{self:?}")),
        }
    }
}

impl MappingFns for () {
    fn iter() -> impl Iterator<Item = Self> + ExactSizeIterator {
        std::iter::empty()
    }

    fn mapping_fn(self) -> Box<dyn MappingFn> {
        unreachable!("`()` is not intended to be used as a mapping function name, but an indicator that no mapping functions are used.")
    }
}
