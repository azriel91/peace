use std::ops::{Deref, DerefMut};

use peace_resources_rt::type_reg::{
    untagged::{BoxDataTypeDowncast, DataType, DataTypeWrapper, FromDataType},
    TypeNameLit,
};
use serde::Serialize;

use crate::{AnySpecDataType, AnySpecRt};

/// Box of a [`DataType`] that is also a [`ValueSpecRt`].
#[derive(Clone, Debug, Serialize)]
pub struct AnySpecRtBoxed(pub(crate) Box<dyn AnySpecDataType>);

impl AnySpecRtBoxed {
    /// Returns a new `ValueSpecRtBoxed` wrapper around the provided type.
    pub fn new<T>(t: T) -> Self
    where
        T: DataType + AnySpecRt,
    {
        Self(Box::new(t))
    }

    /// Returns the inner `Box<dyn ValueSpecDataType>`.
    pub fn into_inner(self) -> Box<dyn AnySpecDataType> {
        self.0
    }
}

impl Deref for AnySpecRtBoxed {
    type Target = dyn AnySpecDataType;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl DerefMut for AnySpecRtBoxed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut()
    }
}

impl<T> FromDataType<T> for AnySpecRtBoxed
where
    T: DataType + AnySpecRt,
{
    fn from(t: T) -> Self {
        AnySpecRtBoxed(Box::new(t))
    }
}

impl<T> BoxDataTypeDowncast<T> for AnySpecRtBoxed
where
    T: DataType + AnySpecRt,
{
    fn downcast_ref(&self) -> Option<&T> {
        self.0.downcast_ref::<T>()
    }

    fn downcast_mut(&mut self) -> Option<&mut T> {
        self.0.downcast_mut::<T>()
    }
}

impl DataTypeWrapper for AnySpecRtBoxed {
    fn type_name(&self) -> TypeNameLit {
        DataType::type_name(&*self.0)
    }

    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    fn debug(&self) -> &dyn std::fmt::Debug {
        &self.0
    }

    fn inner(&self) -> &dyn DataType {
        &self.0
    }
}
