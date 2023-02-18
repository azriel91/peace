use std::{fmt::Debug, hash::Hash};

use peace_rt_model::cmd_context_params::ProfileParams;
use serde::{de::DeserializeOwned, Serialize};

/// The application does not use any workspace parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileParamsNone;

/// The application has workspace parameters.
#[derive(Debug)]
pub struct ProfileParamsSome<ProfileParamsK>(pub(crate) Option<ProfileParams<ProfileParamsK>>)
where
    ProfileParamsK:
        Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static;
