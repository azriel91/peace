use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

/// Builder for an `<ItemSpec::Params as Params>::Spec`
pub trait ParamsSpecBuilder {
    type Output: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static;
}
