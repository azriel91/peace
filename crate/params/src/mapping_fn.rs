use std::fmt::Debug;

use peace_resource_rt::{
    resources::ts::SetUp,
    type_reg::untagged::{BoxDt, DataType},
    Resources,
};
use serde::{Serialize, Serializer};

use crate::{MappingFnImpl, ParamsResolveError, ValueResolutionCtx};

/// Type erased mapping function.
///
/// This is used by Peace to hold type-erased mapping functions, and is not
/// intended to be implemented by users or implementors.
pub trait MappingFn: Debug + DataType {
    /// Returns a type-erased `MappingFn` that wraps the given function.
    ///
    /// This allows different types of logic to be held as a common type.
    ///
    /// # Implementors
    ///
    /// This function is not intended to be overwritten -- perhaps it should be
    /// placed in a sealed trait.
    fn new<T, F, Args>(field_name: Option<String>, f: F) -> Box<dyn MappingFn>
    where
        MappingFnImpl<T, F, Args>: From<(Option<String>, F)> + MappingFn,
        Self: Sized,
    {
        let mapping_fn = MappingFnImpl::from((field_name, f));
        Box::new(mapping_fn)
    }

    /// Maps data in resources to the output type, used for `Item::Params`.
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
        field_name: Option<&str>,
    ) -> Result<BoxDt, ParamsResolveError>;

    /// Maps data in resources to the output type, used for `Item::Params`.
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
        field_name: Option<&str>,
    ) -> Result<Option<BoxDt>, ParamsResolveError>;

    /// Returns whether this mapping function actually holds the function logic.
    ///
    /// Deserialized mapping functions will not hold any function logic, and
    /// Peace uses this function to determine if this is an empty `MappingFn`.
    fn is_valued(&self) -> bool;
}

impl Clone for Box<dyn MappingFn> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

impl Serialize for dyn MappingFn + '_ {
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
