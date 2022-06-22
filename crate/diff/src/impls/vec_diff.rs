use std::{
    cmp::{max, min},
    fmt::{Debug, Formatter, Result as FmtResult},
};

use serde::{Deserialize, Serialize};

use crate::Diff;

/// The type of change to make to a vec
#[derive(Serialize, Deserialize)]
#[serde(bound(serialize = "T::Repr: Serialize"))]
#[serde(bound(deserialize = "T::Repr: Deserialize<'de>"))]
pub enum VecDiffType<T: Diff> {
    Removed { index: usize, len: usize },
    Altered { index: usize, changes: Vec<T::Repr> },
    Inserted { index: usize, changes: Vec<T::Repr> },
}

impl<T: Diff> Debug for VecDiffType<T>
where
    T::Repr: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            VecDiffType::Removed { index, len } => f
                .debug_struct("Removed")
                .field("index", index)
                .field("len", len)
                .finish(),
            VecDiffType::Altered { index, changes } => f
                .debug_struct("Altered")
                .field("index", index)
                .field("changes", changes)
                .finish(),
            VecDiffType::Inserted { index, changes } => f
                .debug_struct("Inserted")
                .field("index", index)
                .field("changes", changes)
                .finish(),
        }
    }
}

impl<T: Diff> PartialEq for VecDiffType<T>
where
    T::Repr: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                VecDiffType::Removed { index, len },
                VecDiffType::Removed {
                    index: ref index_,
                    len: ref len_,
                },
            ) => index == index_ && len == len_,
            (
                VecDiffType::Altered { index, changes },
                VecDiffType::Altered {
                    index: ref index_,
                    changes: ref changes_,
                },
            ) => index == index_ && changes == changes_,
            (
                VecDiffType::Inserted { index, changes },
                VecDiffType::Inserted {
                    index: ref index_,
                    changes: ref changes_,
                },
            ) => index == index_ && changes == changes_,
            _ => false,
        }
    }
}

impl<T: Diff> Clone for VecDiffType<T>
where
    T::Repr: Clone,
{
    fn clone(&self) -> Self {
        match self {
            VecDiffType::Removed { index, len } => VecDiffType::Removed {
                index: *index,
                len: *len,
            },
            VecDiffType::Altered { index, changes } => VecDiffType::Altered {
                index: *index,
                changes: changes.clone(),
            },
            VecDiffType::Inserted { index, changes } => VecDiffType::Inserted {
                index: *index,
                changes: changes.clone(),
            },
        }
    }
}

/// The collection of difference-vec's
#[derive(Serialize, Deserialize)]
#[serde(bound(serialize = "T::Repr: Serialize"))]
#[serde(bound(deserialize = "T::Repr: Deserialize<'de>"))]
pub struct VecDiff<T: Diff>(pub Vec<VecDiffType<T>>);

impl<T: Diff + PartialEq> Diff for Vec<T> {
    type Repr = VecDiff<T>;

    fn diff(&self, other: &Self) -> Self::Repr {
        let mut changes = Vec::new();
        let mut pos_x = 0;
        let mut pos_y = 0;
        loop {
            let (is_match, deletions, insertions) = find_match(&self[pos_x..], &other[pos_y..]);

            // TODO: simplify logic here
            if deletions == 0 || insertions == 0 {
                if deletions > 0 {
                    changes.push(VecDiffType::Removed {
                        index: pos_x,
                        len: deletions,
                    });
                } else if insertions > 0 {
                    changes.push(VecDiffType::Inserted {
                        index: pos_x,
                        changes: other[pos_y..pos_y + insertions]
                            .iter()
                            .map(|new| T::identity().diff(new))
                            .collect(),
                    });
                }
            } else if deletions == insertions {
                changes.push(VecDiffType::Altered {
                    index: pos_x,
                    changes: self[pos_x..pos_x + deletions]
                        .iter()
                        .zip(other[pos_y..pos_y + insertions].iter())
                        .map(|(a, b)| a.diff(b))
                        .collect(),
                });
            } else if deletions > insertions {
                changes.push(VecDiffType::Altered {
                    index: pos_x,
                    changes: self[pos_x..pos_x + insertions]
                        .iter()
                        .zip(other[pos_y..pos_y + insertions].iter())
                        .map(|(a, b)| a.diff(b))
                        .collect(),
                });
                changes.push(VecDiffType::Removed {
                    index: pos_x + insertions,
                    len: deletions - insertions,
                });
            } else {
                changes.push(VecDiffType::Altered {
                    index: pos_x,
                    changes: self[pos_x..pos_x + deletions]
                        .iter()
                        .zip(other[pos_y..pos_y + deletions].iter())
                        .map(|(a, b)| a.diff(b))
                        .collect(),
                });
                changes.push(VecDiffType::Inserted {
                    index: pos_x + deletions,
                    changes: other[pos_y + deletions..pos_y + insertions]
                        .iter()
                        .map(|new| T::identity().diff(new))
                        .collect(),
                });
            }

            if is_match {
                pos_x += deletions + 1;
                pos_y += insertions + 1;
            } else {
                break;
            }
        }
        VecDiff(changes)
    }

    fn apply(&mut self, diff: &Self::Repr) {
        let mut relative_index = 0_isize;
        for change in &diff.0 {
            match change {
                VecDiffType::Removed { index, len } => {
                    let index = (*index as isize + relative_index) as usize;
                    self.drain(index..index + len);
                    relative_index -= *len as isize;
                }
                VecDiffType::Inserted { index, changes } => {
                    let index = (*index as isize + relative_index) as usize;
                    self.splice(
                        index..index,
                        changes.iter().map(|d| T::identity().apply_new(d)),
                    );
                    relative_index += changes.len() as isize;
                }
                VecDiffType::Altered { index, changes } => {
                    let index = (*index as isize + relative_index) as usize;
                    let range = index..index + changes.len();
                    for (value, diff) in self[range].iter_mut().zip(changes.iter()) {
                        value.apply(diff);
                    }
                }
            }
        }
    }

    fn identity() -> Self {
        Vec::new()
    }
}

impl<T: Diff> Debug for VecDiff<T>
where
    T::Repr: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl<T: Diff> PartialEq for VecDiff<T>
where
    T::Repr: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Diff> Clone for VecDiff<T>
where
    T::Repr: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

/// Finds the closest-to-starting element present in both slices.
///
/// Returns whether an equal element was found in each slice, the index of the
/// element in a, the index in b. If no match was found between a and b, returns
/// the indices of the end of each slice, respectively.
fn find_match<T: PartialEq>(a: &[T], b: &[T]) -> (bool, usize, usize) {
    let (mut x, mut y) = (0, 0);
    let mut found_match = false;
    if !a.is_empty() && !b.is_empty() {
        let max_depth = a.len() + b.len() - 1;
        for depth in 0..max_depth {
            let x_lower_bound = max(depth as isize - b.len() as isize + 1, 0) as usize;
            x = min(depth, a.len() - 1);
            loop {
                y = depth - x;
                if a[x] == b[y] {
                    found_match = true;
                    break;
                }
                if x > x_lower_bound {
                    x -= 1;
                } else {
                    break;
                }
            }

            if found_match {
                break;
            }
        }
    }
    if !found_match {
        x = a.len();
        y = b.len();
    }
    (found_match, x, y)
}
