#![allow(clippy::multiple_bound_locations)] // https://github.com/marcianx/downcast-rs/issues/19

use std::{any::Any, fmt};

use downcast_rs::DowncastSync;
use dyn_clone::DynClone;
use peace_resource_rt::type_reg::untagged::DataType;

use crate::AnySpecRt;

/// A [`DataType`] that is also an [`AnySpecRt`].
pub trait AnySpecDataType: AnySpecRt + DataType + DowncastSync {}

impl<T> AnySpecDataType for T where
    T: Any + DynClone + fmt::Debug + AnySpecRt + erased_serde::Serialize + Send + Sync
{
}

downcast_rs::impl_downcast!(sync AnySpecDataType);

impl Clone for Box<dyn AnySpecDataType> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(self.as_ref())
    }
}

impl serde::Serialize for dyn AnySpecDataType + '_ {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        erased_serde::serialize(self, serializer)
    }
}
