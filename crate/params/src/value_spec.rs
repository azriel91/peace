use std::fmt::{self, Debug};

use serde::{Deserialize, Serialize};

use crate::{MappingFn, MappingFnImpl};

/// How to populate a field's value in an item spec's params.
///
/// The `FromMap` variant's mapping function is `None` when deserialized, as it
/// is impossible to determine the underlying `F` and `U` type parameters for
/// the backing `MappingFnImpl`.
///
/// For deserialization:
///
/// 1. A `ParamsSpecsTypeReg` is constructed, and deserialization functions are
///    registered from `ItemSpecId` to `ValueSpecDe<T, F, U>`, where `F` and `U`
///    are derived from the `ParamsSpec` provided by the user.
///
/// 2. `value_specs.yaml` is deserialized using that type registry.
///
/// 3. Each `ValueSpecDe<T>` is mapped into a `ValueSpec<T>`, and
///    subsequently `BoxDt` to be passed around in a `CmdCtx`.
///
/// 4. These `BoxDt`s are downcasted back to `ValueSpec<T>` when resolving
///    values for item spec params and params partials.
#[derive(Clone, Serialize, Deserialize)]
#[serde(from = "crate::ValueSpecDe<T>")]
pub enum ValueSpec<T>
where
    T: Clone + fmt::Debug + Send + Sync + 'static,
{
    /// Use this provided value.
    Value(T),
    /// Look up the value populated by a predecessor.
    From,
    /// Look up some data populated by a predecessor, and compute the value
    /// from that data.
    FromMap(Box<dyn MappingFn<Output = T>>),
}

impl<T> ValueSpec<T>
where
    T: Clone + fmt::Debug + Send + Sync + 'static,
{
    pub fn from_map<F, U>(f: F) -> Self
    where
        F: Fn(&U) -> Option<T> + Clone + Send + Sync + 'static,
        U: Clone + Debug + Send + Sync + 'static,
    {
        let mapping_fn = MappingFnImpl::from(f);
        Self::FromMap(Box::new(mapping_fn))
    }
}

impl<T> fmt::Debug for ValueSpec<T>
where
    T: Clone + fmt::Debug + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(t) => f.debug_tuple("Value").field(t).finish(),
            Self::From => f.write_str("From"),
            Self::FromMap(_) => f.debug_tuple("FromMap").field(&"..").finish(),
        }
    }
}
