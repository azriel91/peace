use std::fmt::{Debug, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};

use crate::Diff;

#[derive(Serialize, Deserialize)]
pub enum OptionDiff<T: Diff> {
    Some(T::Repr),
    None,
    NoChange,
}

impl<T: Diff + PartialEq> Diff for Option<T> {
    type Repr = OptionDiff<T>;

    fn diff(&self, other: &Self) -> Self::Repr {
        match (self, other) {
            (Some(value), Some(other_value)) => {
                if value == other_value {
                    OptionDiff::NoChange
                } else {
                    OptionDiff::Some(value.diff(other_value))
                }
            }
            (Some(_), None) => OptionDiff::None,
            (None, Some(other_value)) => OptionDiff::Some(T::identity().diff(other_value)),
            (None, None) => OptionDiff::NoChange,
        }
    }

    fn apply(&mut self, diff: &Self::Repr) {
        match diff {
            OptionDiff::None => *self = None,
            OptionDiff::Some(change) => {
                if let Some(value) = self {
                    value.apply(change);
                } else {
                    *self = Some(T::identity().apply_new(change))
                }
            }
            _ => {}
        }
    }

    fn identity() -> Self {
        None
    }
}

impl<T: Diff> Debug for OptionDiff<T>
where
    T::Repr: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self {
            OptionDiff::Some(change) => f.debug_tuple("Some").field(change).finish(),
            OptionDiff::None => write!(f, "None"),
            OptionDiff::NoChange => write!(f, "NoChange"),
        }
    }
}

impl<T: Diff> PartialEq for OptionDiff<T>
where
    T::Repr: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OptionDiff::Some(a), OptionDiff::Some(b)) => a == b,
            (OptionDiff::None, OptionDiff::None) => true,
            (OptionDiff::NoChange, OptionDiff::NoChange) => true,
            _ => false,
        }
    }
}

impl<T: Diff> Clone for OptionDiff<T>
where
    T::Repr: Clone,
{
    fn clone(&self) -> Self {
        match self {
            OptionDiff::Some(a) => OptionDiff::Some(a.clone()),
            OptionDiff::None => OptionDiff::None,
            OptionDiff::NoChange => OptionDiff::NoChange,
        }
    }
}
