use std::{
    fmt::{self, Debug},
    marker::PhantomData,
};

use peace_resources::{resources::ts::SetUp, BorrowFail, Resources};
use serde::{Deserialize, Serialize, Serializer};

use crate::MappingFn;

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
    F: Fn(&U) -> T + Clone + Send + Sync + 'static,
    U: Clone + Debug + Send + Sync + 'static,
{
    pub fn map(&self, resources: &Resources<SetUp>) -> Option<T> {
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
            Ok(u) => Some(fn_map(&u)),
            Err(borrow_fail) => match borrow_fail {
                BorrowFail::ValueNotFound => None,
                BorrowFail::BorrowConflictImm | BorrowFail::BorrowConflictMut => panic!(
                    "Failed to borrow `{u}` to map into `{t}`: {borrow_fail:?}.",
                    u = std::any::type_name::<U>(),
                    t = std::any::type_name::<T>(),
                ),
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
            "Fn(&{u}) -> {t}",
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
    F: Fn(&U) -> T + Clone + Send + Sync + 'static,
    U: Clone + Debug + Send + Sync + 'static,
{
    type Output = T;

    fn call(
        &self,
        resources: &Resources<SetUp>,
    ) -> std::option::Option<<Self as MappingFn>::Output> {
        MappingFnImpl::map(self, resources)
    }
}

impl<T, F, U> From<F> for MappingFnImpl<T, F, U>
where
    T: Clone + Debug + Send + Sync + 'static,
    F: Fn(&U) -> T + Clone + Send + Sync + 'static,
    U: Clone + Debug + Send + Sync + 'static,
{
    fn from(fn_map: F) -> Self {
        Self {
            fn_map: Some(fn_map),
            marker: PhantomData,
        }
    }
}
