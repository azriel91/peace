use std::fmt;

use serde::Serialize;

use crate::MappingFn;

/// How to populate a field's value in an item spec's params.
///
/// This does not implement `Deserialize`, as it is impossible to determine the
/// underlying `F` and `U` type parameters for the backing `MappingFnImpl`.
///
/// However, the `ValueSpecDe` type is not type-erased, and so for
/// deserialization:
///
/// 1. A `ValueSpecsTypeReg` is constructed, and deserialization functions are
///    registered from `ItemSpecId` to `ValueSpecDe<T, F, U>`, where `F` and `U`
///    are derived from the `ParamsSpec` provided by the user.
///
/// 2. `value_specs.yaml` is deserialized using that type registry.
///
/// 3. Each `ValueSpecDe<T, F, U>` is mapped into a `ValueSpec<T>`, and
///    subsequently `BoxDt` to be passed around in a `CmdCtx`.
///
/// 4. These `BoxDt`s are downcasted back to `ValueSpec<T>` when resolving
///    values for item spec params and params partials.
#[derive(Clone, Serialize)]
pub enum ValueSpec<T> {
    /// Use this provided value.
    Value(T),
    /// Look up the value populated by a predecessor.
    From,
    /// Look up some data populated by a predecessor, and compute the value
    /// from that data.
    FromMap(Box<dyn MappingFn<Output = T>>),
}

impl<T> fmt::Debug for ValueSpec<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(t) => f.debug_tuple("Value").field(t).finish(),
            Self::From => f.write_str("From"),
            Self::FromMap(_) => f.debug_tuple("FromMap").field(&"..").finish(),
        }
    }
}
