use std::fmt::{self, Debug};

use serde::{Deserialize, Serialize};

use crate::{MappingFnImpl, ParamsFieldless, ParamsSpecFieldless};

type FnPlaceholder<T> = fn(&()) -> Option<T>;

/// Exists to deserialize `FromMap` with a non-type-erased `MappingFnImpl`
#[derive(Clone, Serialize, Deserialize)]
pub enum ParamsSpecFieldlessDe<T>
where
    T: ParamsFieldless,
{
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
    Value(T),
    /// Uses a value loaded from `resources` at runtime.
    ///
    /// The value may have been provided by workspace params, or
    /// inserted by a predecessor at runtime.
    From,
    /// Look up some data populated by a predecessor, and compute the value
    /// from that data.
    FromMap(MappingFnImpl<T, FnPlaceholder<T>, ((),)>),
}

impl<T> Debug for ParamsSpecFieldlessDe<T>
where
    T: ParamsFieldless + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stored => f.write_str("Stored"),
            Self::Value(t) => f.debug_tuple("Value").field(t).finish(),
            Self::From => f.write_str("From"),
            Self::FromMap(mapping_fn_impl) => {
                f.debug_tuple("FromMap").field(&mapping_fn_impl).finish()
            }
        }
    }
}

impl<T> From<ParamsSpecFieldlessDe<T>> for ParamsSpecFieldless<T>
where
    T: ParamsFieldless + Clone + Debug + Send + Sync + 'static,
{
    fn from(value_spec_de: ParamsSpecFieldlessDe<T>) -> Self {
        match value_spec_de {
            ParamsSpecFieldlessDe::Stored => ParamsSpecFieldless::Stored,
            ParamsSpecFieldlessDe::Value(t) => ParamsSpecFieldless::Value(t),
            ParamsSpecFieldlessDe::From => ParamsSpecFieldless::From,
            ParamsSpecFieldlessDe::FromMap(mapping_fn_impl) => {
                ParamsSpecFieldless::FromMap(Box::new(mapping_fn_impl))
            }
        }
    }
}
