#![cfg(test)]

pub(crate) use crate::{
    fn_invocation::FnInvocation,
    fn_name::fn_name_short,
    fn_tracker_output::FnTrackerOutput,
    no_op_output::NoOpOutput,
    vec_copy_item_spec::{
        VecA, VecB, VecCopyDiff, VecCopyError, VecCopyItemSpec, VecCopyItemSpecWrapper,
        VecCopyState,
    },
};

mod cfg;
mod data;
mod diff;
mod resources;
mod rt;
mod rt_model;

mod fn_invocation;
mod fn_name;
mod fn_tracker_output;
mod no_op_output;
mod vec_copy_item_spec;
