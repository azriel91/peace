use std::fmt;

use peace_resources::{resources::ts::SetUp, Resources};

pub enum ValueSpec<T> {
    Value(T),
    From,
    FromMap(Box<dyn (Fn(&Resources<SetUp>) -> Option<T>) + Send + Sync + 'static>),
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
