use std::fmt::{self, Debug};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{MappingFnImpl, ValueSpec};

type FnPlaceholder<T> = fn(&()) -> Option<T>;

/// Exists to deserialize `MappingFn` with a non-type-erased `MappingFnImpl`
#[derive(Clone, Deserialize)]
pub enum ValueSpecDe<T> {
    /// Loads a stored value spec.
    ///
    /// The value used is determined by the value spec that was
    /// last stored in the `params_specs_file`. This means it
    /// could be loaded as a `Value(T)` during context `build()`.
    ///
    /// This variant may be provided when defining a command context
    /// builder. However, this variant is never serialized, but
    /// whichever value was *first* stored is re-loaded then
    /// re-serialized.
    ///
    /// If no value spec was previously serialized, then the command
    /// context build will return an error.
    Stored,
    /// Uses the provided value.
    ///
    /// The value used is whatever is passed in to the command context
    /// builder.
    Value {
        /// The value to use.
        value: T,
    },
    /// Uses a value loaded from `resources` at runtime.
    ///
    /// The value may have been provided by workspace params, or
    /// inserted by a predecessor at runtime.
    InMemory,
    /// Look up some data populated by a predecessor, and compute the value
    /// from that data.
    MappingFn(MappingFnImpl<T, FnPlaceholder<T>, ((),)>),
}

impl<T> Debug for ValueSpecDe<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stored => f.write_str("Stored"),
            Self::Value { value } => f.debug_tuple("Value").field(value).finish(),
            Self::InMemory => f.write_str("InMemory"),
            Self::MappingFn(mapping_fn_impl) => {
                f.debug_tuple("MappingFn").field(&mapping_fn_impl).finish()
            }
        }
    }
}

impl<T> From<ValueSpecDe<T>> for ValueSpec<T>
where
    T: Clone + Debug + DeserializeOwned + Serialize + Send + Sync + 'static,
{
    fn from(value_spec_de: ValueSpecDe<T>) -> Self {
        match value_spec_de {
            ValueSpecDe::Stored => ValueSpec::Stored,
            ValueSpecDe::Value { value } => ValueSpec::Value { value },
            ValueSpecDe::InMemory => ValueSpec::InMemory,
            ValueSpecDe::MappingFn(mapping_fn_impl) => {
                ValueSpec::MappingFn(Box::new(mapping_fn_impl))
            }
        }
    }
}
