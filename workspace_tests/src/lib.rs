#![cfg(test)]

pub(crate) use crate::{
    fn_invocation::FnInvocation,
    fn_name::fn_name_short,
    fn_tracker_output::FnTrackerOutput,
    fn_tracker_presenter::FnTrackerPresenter,
    no_op_output::NoOpOutput,
    vec_copy_item_spec::{
        VecA, VecB, VecCopyDiff, VecCopyError, VecCopyItemSpec, VecCopyItemSpecWrapper,
        VecCopyState,
    },
};

// `peace` test modules
mod cfg;
mod data;
mod diff;
mod fmt;
mod resources;
mod rt;
mod rt_model;

// `peace_item_specs` test modules
#[cfg(feature = "item_specs")]
mod item_specs;

// `workspace_tests` support code
mod fn_invocation;
mod fn_name;
mod fn_tracker_output;
mod fn_tracker_presenter;
mod no_op_output;
mod vec_copy_item_spec;
