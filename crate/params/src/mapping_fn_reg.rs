use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{MappingFn, MappingFnName, MappingFns};

/// Map of serializable [`MappingFns`] to each [`MappingFn`] logic.
///
/// This is intended to be called by the Peace framework for each tool
/// implementor's [`MappingFns`] implementation.
#[derive(Debug)]
pub struct MappingFnReg(HashMap<MappingFnName, Box<dyn MappingFn>>);

impl MappingFnReg {
    /// Returns a new `MappingFnRegistry`.
    pub fn new() -> Self {
        MappingFnReg(HashMap::new())
    }

    /// Returns a new `MappingFnRegistry` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        MappingFnReg(HashMap::with_capacity(capacity))
    }

    /// Registers a single `MappingFns` variant with this registry.
    ///
    /// This is a convenience function for `mapping_fn_reg.insert(m_fns.name(),
    /// m_fns.mapping_fn());`
    pub fn register<MFns>(&mut self, m_fns: MFns)
    where
        MFns: MappingFns,
    {
        self.insert(m_fns.name(), m_fns.mapping_fn());
    }

    /// Registers all `MappingFns` from `MFns` with this registry.
    ///
    /// This is a convenience function for `MFns::iter().for_each(|m_fns|
    /// mapping_fn_reg.register::<MFns>());`
    pub fn register_all<MFns>(&mut self)
    where
        MFns: MappingFns,
    {
        MFns::iter().for_each(|m_fns| self.register(m_fns));
    }
}

impl Deref for MappingFnReg {
    type Target = HashMap<MappingFnName, Box<dyn MappingFn>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MappingFnReg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for MappingFnReg {
    fn default() -> Self {
        Self::new()
    }
}
