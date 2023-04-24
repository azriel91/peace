use std::fmt::Debug;

/// Builder for an `<ItemSpec::Params as Params>::Spec`
pub trait ParamsSpecBuilder {
    // Clone + Serialize + DeserializeOwned
    type Output: Debug + Send + Sync + 'static;
}
