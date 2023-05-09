use std::{
    fmt::{self, Debug},
    marker::PhantomData,
};

use peace_resources::{resources::ts::SetUp, BorrowFail, Resources};
use serde::{Deserialize, Serialize, Serializer};

use crate::{FieldNameAndType, MappingFn, ParamsResolveError, ValueResolutionCtx};

/// Wrapper around a mapping function so that it can be serialized.
#[derive(Clone, Serialize, Deserialize)]
pub struct MappingFnImpl<T, F, U> {
    /// This field's name within its parent struct.
    ///
    /// `None` if this is the top level value type.
    field_name: Option<String>,
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
            .field("field_name", &self.field_name)
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
    pub fn new(field_name: Option<String>, fn_map: F) -> Self {
        Self {
            fn_map: Some(fn_map),
            field_name,
            marker: PhantomData,
        }
    }

    pub fn map(
        &self,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<T, ParamsResolveError> {
        if let Some(field_name) = self.field_name.as_deref() {
            value_resolution_ctx.push(FieldNameAndType::new(
                field_name.to_string(),
                std::any::type_name::<T>().to_string(),
            ));
        }
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
            Ok(u) => fn_map(&u).ok_or(ParamsResolveError::FromMap {
                value_resolution_ctx: value_resolution_ctx.clone(),
                from_type_name: std::any::type_name::<U>(),
            }),
            Err(borrow_fail) => match borrow_fail {
                BorrowFail::ValueNotFound => Err(ParamsResolveError::FromMap {
                    value_resolution_ctx: value_resolution_ctx.clone(),
                    from_type_name: std::any::type_name::<U>(),
                }),
                BorrowFail::BorrowConflictImm | BorrowFail::BorrowConflictMut => {
                    Err(ParamsResolveError::FromMapBorrowConflict {
                        value_resolution_ctx: value_resolution_ctx.clone(),
                        from_type_name: std::any::type_name::<U>(),
                    })
                }
            },
        }
    }

    pub fn try_map(
        &self,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<Option<T>, ParamsResolveError> {
        if let Some(field_name) = self.field_name.as_deref() {
            value_resolution_ctx.push(FieldNameAndType::new(
                field_name.to_string(),
                std::any::type_name::<T>().to_string(),
            ));
        }
        let Some(fn_map) = self.fn_map.as_ref() else {
            panic!("`MappingFnImpl::try_map` called when `fn_map` is `None`.\n\
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
                        value_resolution_ctx: value_resolution_ctx.clone(),
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
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<<Self as MappingFn>::Output, ParamsResolveError> {
        MappingFnImpl::map(self, resources, value_resolution_ctx)
    }

    fn try_map(
        &self,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<Option<<Self as MappingFn>::Output>, ParamsResolveError> {
        MappingFnImpl::try_map(self, resources, value_resolution_ctx)
    }
}
