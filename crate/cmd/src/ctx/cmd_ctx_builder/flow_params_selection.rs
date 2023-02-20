use std::{fmt::Debug, hash::Hash};

use peace_rt_model::cmd_context_params::FlowParams;
use serde::{de::DeserializeOwned, Serialize};

/// The application does not use any flow parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowParamsNone;

/// The application has flow parameters.
#[derive(Debug)]
pub struct FlowParamsSome<FlowParamsK>(pub(crate) Option<FlowParams<FlowParamsK>>)
where
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static;
