use std::fmt::{self, Debug};

use serde::Deserialize;

use crate::{MappingFnImpl, ValueSpec};

/// Exists to deserialize `FromMap` with a non-type-erased `MappingFnImpl`
#[derive(Clone, Deserialize)]
pub enum ValueSpecDe<T, F, U> {
    /// Use this provided value.
    Value(T),
    /// Look up the value populated by a predecessor.
    From,
    /// Look up some data populated by a predecessor, and compute the value
    /// from that data.
    FromMap(MappingFnImpl<T, F, U>),
}

impl<T, F, U> fmt::Debug for ValueSpecDe<T, F, U>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(t) => f.debug_tuple("Value").field(t).finish(),
            Self::From => f.write_str("From"),
            Self::FromMap(mapping_fn_impl) => {
                f.debug_tuple("FromMap").field(&mapping_fn_impl).finish()
            }
        }
    }
}

impl<T, F, U> From<ValueSpecDe<T, F, U>> for ValueSpec<T>
where
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(&U) -> T + Clone + Send + Sync + 'static,
    U: Clone + Debug + Send + Sync + 'static,
{
    fn from(value_spec_de: ValueSpecDe<T, F, U>) -> Self {
        match value_spec_de {
            ValueSpecDe::Value(t) => ValueSpec::Value(t),
            ValueSpecDe::From => ValueSpec::From,
            ValueSpecDe::FromMap(mapping_fn_impl) => ValueSpec::FromMap(Box::new(mapping_fn_impl)),
        }
    }
}
