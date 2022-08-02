use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
    ops::ControlFlow,
};

use crate::Equality;

/// Types that may be equal.
///
/// This trait and [`Equality`] represent when two values are known to exist,
/// but their content is not known.
pub trait MaybeEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    fn maybe_eq(&self, other: &Rhs) -> Equality;
}

macro_rules! maybe_eq_impl {
    ($($name:tt,)+) => {
        $(
            impl MaybeEq for $name {
                fn maybe_eq(&self, other: &Self) -> Equality {
                    Equality::from(self == other)
                }
            }
        )+
    };
}

macro_rules! maybe_eq_impl_list {
    ($($name:tt<$param:ident>,)+) => {
        $(
            impl<$param> MaybeEq for $name<$param>
            where $param: PartialEq {
                /// Note that this implementation uses `PartialEq` to determine [`Equality`].
                ///
                /// There is no attempt to match elements together then do an element-wise
                /// comparison, as the order of elements affects comparison, and it is up to consumers to manually
                /// implement that level of accuracy.
                fn maybe_eq(&self, other: &Self) -> Equality {
                    Equality::from(self == other)
                }
            }
        )+
    };
}

macro_rules! maybe_eq_impl_set {
    ($($name:tt<$param:ident>,)+) => {
        $(
            impl<$param> MaybeEq for $name<$param>
            where $param: MaybeEq + Eq + std::hash::Hash {
                fn maybe_eq(&self, other: &Self) -> Equality {
                    Equality::from(self == other)
                }
            }
        )+
    };
}

// We have to choose whether we implement this for `Map<K, V>` where `V:
// MaybeEq` or `V: PartialEq`.
//
// It should be less limiting to do it for the former, as consumers may be
// able to implement `MaybeEq` for `V`. If `V` is not in their control, then
// they wouldn't be able to use `MaybeEq` or `Equality` for their map anyway,
// and so would be using a `PartialEq` comparison.
macro_rules! maybe_eq_impl_map {
    ($($name:tt<$key:ident, $value:ident> $(+ $bound:tt)*,)+) => {
        $(
            impl<$key, $value> MaybeEq for $name<$key, $value>
            where
                $key: Eq + std::hash::Hash $(+ $bound)*,
                $value: MaybeEq {
                fn maybe_eq(&self, other: &Self) -> Equality {
                    if self.len() != other.len() {
                        Equality::NotEqual
                    } else {
                        let equality = self.iter()
                            .try_fold(Equality::Equal, |equality, (k1, v1)| {
                                if let Some(v2) = other.get(k1) {
                                    match v1.maybe_eq(v2) {
                                        Equality::NotEqual => ControlFlow::Break(Equality::NotEqual),
                                        Equality::Equal => ControlFlow::Continue(equality),
                                        Equality::Unknown => ControlFlow::Continue(Equality::Unknown),
                                    }
                                } else {
                                    ControlFlow::Break(Equality::NotEqual)
                                }
                            });

                        // https://github.com/rust-lang/rust/issues/82223
                        match equality {
                            ControlFlow::Continue(equality) | ControlFlow::Break(equality) => equality,
                        }
                    }
                }
            }
        )+
    };
}

maybe_eq_impl! {
    (),
    bool, char, str,
    f32, f64,
    i128, i16, i32, i64, i8, isize,
    u128, u16, u32, u64, u8, usize,
}

maybe_eq_impl_list! {
    Vec<T>, VecDeque<T>, LinkedList<T>,
}

maybe_eq_impl_set! {
    HashSet<T>, BTreeSet<T>,
}

maybe_eq_impl_map! {
    HashMap<K, V>,
    BTreeMap<K, V> + Ord,
}
