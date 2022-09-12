#![cfg(test)]

pub(crate) use crate::{
    no_op_output::NoOpOutput,
    vec_copy_item_spec::{VecA, VecB, VecCopyError, VecCopyItemSpec, VecCopyItemSpecWrapper},
};

mod cfg;
mod data;
mod diff;
mod resources;
mod rt;
mod rt_model;

mod no_op_output;
mod vec_copy_item_spec;
