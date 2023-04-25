use peace_resources::{resources::ts::SetUp, type_reg::untagged::DataType, Resources};
use serde::{Serialize, Serializer};

/// Type erased mapping function.
pub trait MappingFn: DataType {
    /// Type that is output by the function.
    type Output;

    /// Maps data in resources to the output type.
    ///
    /// The data being accessed is defined by the implementation of this
    /// function.
    fn call(&self, resources: &Resources<SetUp>) -> Option<Self::Output>;
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
