use std::{collections::BTreeMap, fmt::Debug, hash::Hash};

use peace_core::Profile;
use peace_rt_model::params::FlowParams;
use serde::{de::DeserializeOwned, Serialize};

/// The application does not use any flow parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowParamsNone;

/// The application has flow parameters.
#[derive(Debug)]
pub struct FlowParamsSome<FlowParamsK>(pub(crate) FlowParams<FlowParamsK>)
where
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static;

/// The application has flow parameters from multiple profiles.
#[derive(Debug)]
pub struct FlowParamsSomeMulti<FlowParamsK>(pub(crate) BTreeMap<Profile, FlowParams<FlowParamsK>>)
where
    FlowParamsK: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static;
