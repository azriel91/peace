#![cfg(test)]

pub(crate) use vec_copy_full_spec::{
    VecA, VecB, VecCopyError, VecCopyFullSpec, VecCopyFullSpecWrapper,
};

mod cfg;
mod data;
mod diff;
mod resources;
mod rt;
mod rt_model;

mod vec_copy_full_spec;
