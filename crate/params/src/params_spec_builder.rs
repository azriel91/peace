use std::fmt::Debug;

use serde::Serialize;

/// Builder for an `<ItemSpec::Params as Params>::Spec`
pub trait ParamsSpecBuilder {
    /// The `ParamsSpec` type, which collects how values are to be resolved.
    ///
    /// `serde::DeserializedOwned` is not a bound, because `ValueSpec` is
    /// generic, and so we cannot include the bound on the `ParamsSpec`. See
    /// `ValueSpec` and `ValueSpecDe` for how deserialization is done.
    type Output: Clone + Debug + Serialize + Send + Sync + 'static;
}
