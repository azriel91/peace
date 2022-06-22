use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::{Debug, Formatter, Result as FmtResult},
    hash::Hash,
};

use crate::Diff;

/// The diff struct used to compare two [HashMap]'s
#[derive(Serialize, Deserialize)]
#[serde(bound(serialize = "V::Repr: Serialize, K: Serialize"))]
#[serde(bound(deserialize = "V::Repr: Deserialize<'de>, K: Deserialize<'de>"))]
pub struct HashMapDiff<K: Hash + Eq, V: Diff> {
    /// Values that are changed or added
    pub altered: HashMap<K, <V as Diff>::Repr>,
    /// Values that are removed
    pub removed: HashSet<K>,
}

/// The diff struct used to compare two [BTreeMap]'s
#[derive(Serialize, Deserialize)]
#[serde(bound(serialize = "V::Repr: Serialize, K: Serialize"))]
#[serde(bound(deserialize = "V::Repr: Deserialize<'de>, K: Deserialize<'de>"))]
pub struct BTreeMapDiff<K: Ord + Eq, V: Diff> {
    /// Values that are changed or added
    pub altered: BTreeMap<K, <V as Diff>::Repr>,
    /// Values that are removed
    pub removed: BTreeSet<K>,
}

macro_rules! diff_map {
    ($ty: ident, $diffty: ident, $diffkey: ident, ($($constraints:tt)*)) => {
        impl<K: $($constraints)*, V: Diff> Diff for $ty<K, V>
        where
            K: Clone,
            V: PartialEq,
        {
            type Repr = $diffty<K, V>;

            fn diff(&self, other: &Self) -> Self::Repr {
                let mut diff = $diffty {
                    altered: $ty::new(),
                    removed: $diffkey::new(),
                };
                // can we do better than this?
                for (key, value) in self {
                    if let Some(other_value) = other.get(key) {
                        // don't store values that don't change
                        if value != other_value {
                            diff.altered.insert(key.clone(), value.diff(other_value));
                        }
                    } else {
                        diff.removed.insert(key.clone());
                    }
                }
                for (key, value) in other {
                    if let None = self.get(key) {
                        diff.altered.insert(key.clone(), V::identity().diff(value));
                    }
                }
                diff
            }

            // basically inexpensive
            fn apply(&mut self, diff: &Self::Repr) {
                diff.removed.iter().for_each(|del| {
                    self.remove(del);
                });
                for (key, change) in &diff.altered {
                    if let Some(original) = self.get_mut(key) {
                        original.apply(change);
                    } else {
                        self.insert(key.clone(), V::identity().apply_new(change));
                    }
                }
            }

            fn identity() -> Self {
                $ty::new()
            }
        }

        impl<K: $($constraints)*, V: Diff> Debug for $diffty<K, V>
        where
            K: Debug,
            V::Repr: Debug,
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.debug_struct(stringify!($diffty))
                    .field("altered", &self.altered)
                    .field("removed", &self.removed)
                    .finish()
            }
        }

        impl<K: $($constraints)*, V: Diff> PartialEq for $diffty<K, V>
        where
            V::Repr: PartialEq,
        {
            fn eq(&self, other: &Self) -> bool {
                self.altered == other.altered && self.removed == other.removed
            }
        }

        impl<K: $($constraints)*, V: Diff> Clone for $diffty<K, V>
        where
            K: Clone,
            V::Repr: Clone,
        {
            fn clone(&self) -> Self {
                $diffty {
                    altered: self.altered.clone(),
                    removed: self.removed.clone(),
                }
            }
        }
    }
}

diff_map!(HashMap, HashMapDiff, HashSet, (Hash + Eq));
diff_map!(BTreeMap, BTreeMapDiff, BTreeSet, (Ord));
