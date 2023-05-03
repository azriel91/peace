use std::{
    fmt::{self, Debug},
    marker::PhantomData,
};

use peace_resources::{resources::ts::SetUp, BorrowFail, Resources};
use serde::{Deserialize, Serialize, Serializer};

use crate::{MappingFn, ParamsResolveError};

/// Wrapper around a mapping function so that it can be serialized.
#[derive(Clone, Serialize, Deserialize)]
pub struct MappingFnImpl<T, F, U> {
    #[serde(
        default = "MappingFnImpl::<T, F, U>::fn_map_none",
        skip_deserializing,
        serialize_with = "MappingFnImpl::<T, F, U>::fn_map_serialize"
    )]
    fn_map: Option<F>,
    /// Marker.
    marker: PhantomData<(T, U)>,
}

impl<T, F, U> Debug for MappingFnImpl<T, F, U>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MappingFnImpl")
            .field("fn_map", &Self::fn_map_stringify())
            .field("marker", &self.marker)
            .finish()
    }
}
impl<T, F, U> MappingFnImpl<T, F, U>
where
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(&U) -> Option<T> + Clone + Send + Sync + 'static,
    U: Clone + Debug + Send + Sync + 'static,
{
    pub fn map(
        &self,
        resources: &Resources<SetUp>,
        params_type_name_fn: fn() -> &'static str,
        field_name: &'static str,
    ) -> Result<T, ParamsResolveError> {
        self.try_map(resources, params_type_name_fn, field_name)
            .transpose()
            .unwrap_or_else(|| {
                Err(ParamsResolveError::FromMap {
                    params_type_name: params_type_name_fn(),
                    field_name,
                    field_type_name: std::any::type_name::<T>(),
                    from_type_name: std::any::type_name::<U>(),
                })
            })
    }

    pub fn try_map(
        &self,
        resources: &Resources<SetUp>,
        params_type_name_fn: fn() -> &'static str,
        field_name: &'static str,
    ) -> Result<Option<T>, ParamsResolveError> {
        let Some(fn_map) = self.fn_map.as_ref() else {
            panic!("`MappingFnImpl::map` called when `fn_map` is `None`.\n\
                This is a bug in the Peace framework.\n\
                \n\
                Type parameters are:\n\
                \n\
                * `T`: {t}\n\
                * `U`: {u}\n\
                ",
                t = std::any::type_name::<T>(),
                u = std::any::type_name::<U>(),
                );
        };
        match resources.try_borrow::<U>() {
            Ok(u) => Ok(fn_map(&u)),
            Err(borrow_fail) => match borrow_fail {
                BorrowFail::ValueNotFound => Ok(None),
                BorrowFail::BorrowConflictImm | BorrowFail::BorrowConflictMut => {
                    Err(ParamsResolveError::FromMapBorrowConflict {
                        params_type_name: params_type_name_fn(),
                        field_name,
                        field_type_name: std::any::type_name::<T>(),
                        from_type_name: std::any::type_name::<U>(),
                    })
                }
            },
        }
    }
}

impl<T, F, U> MappingFnImpl<T, F, U> {
    fn fn_map_serialize<S>(_fn_map: &Option<F>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&Self::fn_map_stringify())
    }

    fn fn_map_stringify() -> String {
        format!(
            "Fn(&{u}) -> Option<{t}>",
            t = std::any::type_name::<T>(),
            u = std::any::type_name::<U>(),
        )
    }

    fn fn_map_none() -> Option<F> {
        None
    }
}

impl<T, F, U> MappingFn for MappingFnImpl<T, F, U>
where
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(&U) -> Option<T> + Clone + Send + Sync + 'static,
    U: Clone + Debug + Send + Sync + 'static,
{
    type Output = T;

    fn map(
        &self,
        resources: &Resources<SetUp>,
        params_type_name_fn: fn() -> &'static str,
        field_name: &'static str,
    ) -> Result<<Self as MappingFn>::Output, ParamsResolveError> {
        MappingFnImpl::map(self, resources, params_type_name_fn, field_name)
    }

    fn try_map(
        &self,
        resources: &Resources<SetUp>,
        params_type_name_fn: fn() -> &'static str,
        field_name: &'static str,
    ) -> Result<Option<<Self as MappingFn>::Output>, ParamsResolveError> {
        MappingFnImpl::try_map(self, resources, params_type_name_fn, field_name)
    }
}

impl<T, F, U> From<F> for MappingFnImpl<T, F, U>
where
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(&U) -> Option<T> + Clone + Send + Sync + 'static,
    U: Clone + Debug + Send + Sync + 'static,
{
    fn from(fn_map: F) -> Self {
        Self {
            fn_map: Some(fn_map),
            marker: PhantomData,
        }
    }
}
