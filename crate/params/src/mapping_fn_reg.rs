use std::{
    any::TypeId,
    collections::HashMap,
    hash::{DefaultHasher, Hasher},
    ops::Deref,
};

use crate::{MappingFn, MappingFnRegKey, MappingFns};

/// Map of serializable [`MappingFns`] to each [`MappingFn`] logic.
///
/// This is intended to be called by the Peace framework for each tool
/// implementor's [`MappingFns`] implementation.
#[derive(Debug)]
pub struct MappingFnReg {
    mapping_fns: HashMap<MappingFnRegKey, Box<dyn MappingFn>>,
}

impl MappingFnReg {
    /// Returns a new `MappingFnRegistry`.
    pub fn new() -> Self {
        MappingFnReg {
            mapping_fns: HashMap::new(),
        }
    }

    /// Returns a new `MappingFnRegistry` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        MappingFnReg {
            mapping_fns: HashMap::with_capacity(capacity),
        }
    }

    /// Stores the logic for a [`MappingFn`] into the registry.
    pub fn insert<MFns>(&mut self, m_fn: MFns, mapping_fn: Box<dyn MappingFn>)
    where
        MFns: MappingFns,
    {
        let mapping_fn_key = MappingFnRegKey {
            type_id: TypeId::of::<MFns>(),
            variant_hash: {
                let mut hasher = DefaultHasher::new();
                m_fn.hash(&mut hasher);
                hasher.finish()
            },
        };
        self.mapping_fns.insert(mapping_fn_key, mapping_fn);
    }

    /// Returns the logic corresponding to the given mapping function variant.
    ///
    /// # Notes
    ///
    /// This should never return `None` -- if it does, it indicates a bug in the
    /// Peace framework for registering mapping functions.
    pub fn get<MFns>(&self, m_fn: MFns) -> Option<&Box<dyn MappingFn>>
    where
        MFns: MappingFns,
    {
        let mapping_fn_key = MappingFnRegKey {
            type_id: TypeId::of::<MFns>(),
            variant_hash: {
                let mut hasher = DefaultHasher::new();
                m_fn.hash(&mut hasher);
                hasher.finish()
            },
        };
        self.mapping_fns.get(&mapping_fn_key)
    }
}

impl Deref for MappingFnReg {
    type Target = HashMap<MappingFnRegKey, Box<dyn MappingFn>>;

    fn deref(&self) -> &Self::Target {
        &self.mapping_fns
    }
}

impl Default for MappingFnReg {
    fn default() -> Self {
        Self::new()
    }
}
