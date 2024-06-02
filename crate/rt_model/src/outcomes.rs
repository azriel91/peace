//! Type that represent outcomes of execution.
//!
//! Types in this module must all be serializable, as this allows execution
//! outcomes to be redisplayed without re-executing commands.

pub use self::{
    item_apply::ItemApply, item_apply_boxed::ItemApplyBoxed, item_apply_partial::ItemApplyPartial,
    item_apply_partial_boxed::ItemApplyPartialBoxed, item_apply_partial_rt::ItemApplyPartialRt,
    item_apply_rt::ItemApplyRt,
};

mod item_apply;
mod item_apply_boxed;
mod item_apply_partial;
mod item_apply_partial_boxed;
mod item_apply_partial_rt;
mod item_apply_rt;

macro_rules! box_data_type_newtype {
    ($ty_name:ident, $trait_path:path) => {
        impl std::fmt::Debug for $ty_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_tuple(stringify!($ty_name)).field(&self.0).finish()
            }
        }

        impl $ty_name {
            /// Returns the inner boxed trait.
            pub fn into_inner(self) -> Box<dyn $trait_path> {
                self.0
            }
        }

        impl std::ops::Deref for $ty_name {
            type Target = dyn $trait_path;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $ty_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<T> peace_resources_rt::type_reg::untagged::BoxDataTypeDowncast<T> for $ty_name
        where
            T: $trait_path,
        {
            fn downcast_ref(&self) -> Option<&T> {
                self.0.as_data_type().downcast_ref::<T>()
            }

            fn downcast_mut(&mut self) -> Option<&mut T> {
                self.0.as_data_type_mut().downcast_mut::<T>()
            }
        }

        impl peace_resources_rt::type_reg::untagged::DataTypeWrapper for $ty_name {
            fn type_name(&self) -> peace_resources_rt::type_reg::TypeNameLit {
                peace_resources_rt::type_reg::untagged::DataType::type_name(&*self.0)
            }

            fn clone(&self) -> Self {
                Self(self.0.clone())
            }

            fn debug(&self) -> &dyn std::fmt::Debug {
                &self.0
            }

            fn inner(&self) -> &dyn peace_resources_rt::type_reg::untagged::DataType {
                &self.0
            }
        }
    };
}

pub(crate) use box_data_type_newtype;
