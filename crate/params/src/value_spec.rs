use std::fmt;

use peace_resources::{resources::ts::SetUp, Resources};

/// Alias for `Fn(&Resources) -> T` with additional constraints.
pub type MappingFn<T> = dyn (Fn(&Resources<SetUp>) -> Option<T>) + Send + Sync + 'static;

/// How to populate a field's value in an item spec's params.
pub enum ValueSpec<T> {
    /// Use this provided value.
    Value(T),
    /// Look up the value populated by a predecessor.
    From,
    /// Look up some data populated by a predecessor, and compute the value
    /// from that data.
    FromMap(Box<MappingFn<T>>),
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
