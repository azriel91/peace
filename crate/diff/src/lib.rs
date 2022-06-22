//! Diff trait for the peace automation framework.
//!
//! This is taken from <https://github.com/BenHall-7/diff-struct>.

// Re-export derive macro.
pub use peace_diff_derive::Diff;

pub use crate::{
    diff::Diff,
    differ::Differ,
    impls::{BTreeMapDiff, HashMapDiff, OptionDiff, VecDiff, VecDiffType},
};

mod diff;
mod differ;
mod impls;
