use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{MappingFn, MappingFnName};

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
