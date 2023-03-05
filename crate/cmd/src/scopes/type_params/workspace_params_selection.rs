use std::{fmt::Debug, hash::Hash};

use peace_rt_model::params::WorkspaceParams;
use serde::{de::DeserializeOwned, Serialize};

/// The application does not use any workspace parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceParamsNone;

/// The application has workspace parameters.
#[derive(Debug)]
pub struct WorkspaceParamsSome<WorkspaceParamsK>(pub(crate) WorkspaceParams<WorkspaceParamsK>)
where
    WorkspaceParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static;
