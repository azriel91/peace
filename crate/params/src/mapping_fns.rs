use std::{fmt::Debug, hash::Hash};

use serde::{de::DeserializeOwned, Serialize};

use crate::MappingFn;

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
pub trait MappingFns: Clone + Debug + Hash + PartialEq + Eq + Serialize + DeserializeOwned {
    fn mapping_fn(self) -> Box<dyn MappingFn>;
}

impl MappingFns for () {
    fn mapping_fn(self) -> Box<dyn MappingFn> {
        panic!("`()` is not intended to be used as a mapping function name, but an indicator that no mapping functions are used.")
    }
}
