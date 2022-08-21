#![cfg(test)]

pub(crate) use vec_copy_item_spec::{
    VecA, VecB, VecCopyError, VecCopyItemSpec, VecCopyItemSpecWrapper,
};

mod cfg;
mod data;
mod diff;
mod resources;
mod rt;
mod rt_model;

mod vec_copy_item_spec;
