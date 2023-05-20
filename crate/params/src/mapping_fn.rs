use std::fmt::Debug;

use peace_resources::{resources::ts::SetUp, type_reg::untagged::DataType, Resources};
use serde::{Serialize, Serializer};

use crate::{ParamsResolveError, ValueResolutionCtx};

/// Type erased mapping function.
///
/// This is used by Peace to hold type-erased mapping functions, and is not
/// intended to be implemented by users or implementors.
pub trait MappingFn: Debug + DataType {
    /// Type that is output by the function.
    type Output;

    /// Maps data in resources to the output type.
    ///
    /// The data being accessed is defined by the implementation of this
    /// function.
    ///
    /// # Parameters
    ///
    /// * `resources`: Resources to resolve values from.
    /// * `value_resolution_ctx`: Fields traversed during this value resolution.
    fn map(
        &self,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<Self::Output, ParamsResolveError>;

    /// Maps data in resources to the output type.
    ///
    /// The data being accessed is defined by the implementation of this
    /// function.
    ///
    /// # Parameters
    ///
    /// * `resources`: Resources to resolve values from.
    /// * `value_resolution_ctx`: Fields traversed during this value resolution.
    fn try_map(
        &self,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<Option<Self::Output>, ParamsResolveError>;

    /// Returns whether this mapping function actually holds the function logic.
    ///
    /// Deserialized mapping functions will not hold any function logic, and
    /// Peace uses this function to determine if this is an empty `MappingFn`.
    fn is_valued(&self) -> bool;
}

impl<T> Clone for Box<dyn MappingFn<Output = T>> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

impl<'a, T> Serialize for dyn MappingFn<Output = T> + 'a {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Sadly the following doesn't work, it says the lifetime of:
        // `&'1 self` must outlive `'static`
        //
        // let data_type: &(dyn DataType + 'a) = &self;
        // Serialize::serialize(data_type, serializer)

        // so we have to depend on `erased_serde` directly
        erased_serde::serialize(self, serializer)
    }
}
