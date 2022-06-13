pub use self::{
    map_diff::{BTreeMapDiff, HashMapDiff},
    option_diff::OptionDiff,
    vec_diff::{VecDiff, VecDiffType},
};

mod arc_diff;
mod bool_diff;
mod char_diff;
mod float_diff;
mod int_diff;
mod map_diff;
mod option_diff;
mod string_diff;
mod tuple_diff;
mod vec_diff;
