use std::fmt::{self, Debug};

use peace_resources::{resources::ts::SetUp, BorrowFail, Resources};
use serde::{Deserialize, Serialize};

use crate::{
    AnySpecDataType, AnySpecRt, MappingFn, MappingFnImpl, ParamsResolveError, ValueResolutionCtx,
    ValueSpecRt,
};

/// How to populate a field's value in an item's params.
///
/// The `MappingFn` variant's mapping function is `None` when deserialized, as
/// it is impossible to determine the underlying `F` and `U` type parameters for
/// the backing `MappingFnImpl`.
///
/// For deserialization:
///
/// 1. A `ParamsSpecsTypeReg` is constructed, and deserialization functions are
///    registered from `ItemId` to `ValueSpecDe<T, F, U>`, where `F` and `U` are
///    derived from the `ValueSpec` provided by the user.
///
/// 2. `value_specs.yaml` is deserialized using that type registry.
///
/// 3. Each `ValueSpecDe<T>` is mapped into a `ValueSpec<T>`, and subsequently
///    `AnySpecRtBoxed` to be passed around in a `CmdCtx`.
///
/// 4. These `AnySpecRtBoxed`s are downcasted back to `ValueSpec<T>` when
///    resolving values for item params and params partials.
#[derive(Clone, Serialize, Deserialize)]
#[serde(from = "crate::ValueSpecDe<T>")]
pub enum ValueSpec<T>
where
    T: Clone + Debug + Send + Sync + 'static,
{
    /// Loads a stored value spec.
    ///
    /// The value used is determined by the value spec that was
    /// last stored in the `params_specs_file`. This means it
    /// could be loaded as a `Value(T)` during context `build()`.
    ///
    /// This variant may be provided when defining a command context
    /// builder. However, this variant is never serialized, but
    /// whichever value was *first* stored is re-loaded then
    /// re-serialized.
    ///
    /// If no value spec was previously serialized, then the command
    /// context build will return an error.
    Stored,
    /// Uses a value loaded from `resources` at runtime.
    ///
    /// The value may have been provided by workspace params, or
    /// inserted by a predecessor at runtime.
    InMemory,
    /// Uses a mapped value loaded from `resources` at runtime.
    ///
    /// The value may have been provided by workspace params, or
    /// inserted by a predecessor at runtime, and is mapped by the
    /// given function.
    ///
    /// This is serialized as `MappingFn` with a string value. For
    /// deserialization, there is no actual backing function, so
    /// the user must provide the `MappingFn` in subsequent command
    /// context builds.
    MappingFn(Box<dyn MappingFn<Output = T>>),
    /// Uses the provided value.
    ///
    /// The value used is whatever is passed in to the command context
    /// builder.
    #[serde(untagged)]
    Value(T),
}

impl<T> ValueSpec<T>
where
    T: Clone + Debug + Send + Sync + 'static,
{
    pub fn from_map<F, Args>(field_name: Option<String>, f: F) -> Self
    where
        MappingFnImpl<T, F, Args>: From<(Option<String>, F)> + MappingFn<Output = T>,
    {
        let mapping_fn = MappingFnImpl::from((field_name, f));
        Self::MappingFn(Box::new(mapping_fn))
    }
}

impl<T> Debug for ValueSpec<T>
where
    T: Clone + Debug + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stored => f.write_str("Stored"),
            Self::InMemory => f.write_str("InMemory"),
            Self::MappingFn(mapping_fn) => f.debug_tuple("MappingFn").field(mapping_fn).finish(),
            Self::Value(value) => f.debug_tuple("Value").field(value).finish(),
        }
    }
}

impl<T> From<T> for ValueSpec<T>
where
    T: Clone + Debug + Send + Sync + 'static,
{
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}

impl<T> ValueSpec<T>
where
    T: Clone + Debug + Send + Sync + 'static,
{
    pub fn resolve(
        &self,
        resources: &Resources<peace_resources::resources::ts::SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<T, ParamsResolveError> {
        match self {
            ValueSpec::Stored | ValueSpec::InMemory => match resources.try_borrow::<T>() {
                Ok(value) => Ok((*value).clone()),
                Err(borrow_fail) => match borrow_fail {
                    BorrowFail::ValueNotFound => Err(ParamsResolveError::InMemory {
                        value_resolution_ctx: value_resolution_ctx.clone(),
                    }),
                    BorrowFail::BorrowConflictImm | BorrowFail::BorrowConflictMut => {
                        Err(ParamsResolveError::InMemoryBorrowConflict {
                            value_resolution_ctx: value_resolution_ctx.clone(),
                        })
                    }
                },
            },
            ValueSpec::MappingFn(mapping_fn) => mapping_fn.map(resources, value_resolution_ctx),
            ValueSpec::Value(value) => Ok(value.clone()),
        }
    }

    pub fn resolve_partial(
        &self,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<Option<T>, ParamsResolveError> {
        match self {
            ValueSpec::Stored | ValueSpec::InMemory => match resources.try_borrow::<T>() {
                Ok(value) => Ok(Some((*value).clone())),
                Err(borrow_fail) => match borrow_fail {
                    BorrowFail::ValueNotFound => Err(ParamsResolveError::InMemory {
                        value_resolution_ctx: value_resolution_ctx.clone(),
                    }),
                    BorrowFail::BorrowConflictImm | BorrowFail::BorrowConflictMut => {
                        Err(ParamsResolveError::InMemoryBorrowConflict {
                            value_resolution_ctx: value_resolution_ctx.clone(),
                        })
                    }
                },
            },
            ValueSpec::MappingFn(mapping_fn) => mapping_fn.try_map(resources, value_resolution_ctx),
            ValueSpec::Value(value) => Ok(Some((*value).clone())),
        }
    }
}

impl<T> AnySpecRt for ValueSpec<T>
where
    T: Clone + Debug + Serialize + Send + Sync + 'static,
{
    fn is_usable(&self) -> bool {
        match self {
            Self::Stored => false,
            Self::MappingFn(mapping_fn) => mapping_fn.is_valued(),
            Self::Value { .. } | Self::InMemory => true,
        }
    }

    fn merge(&mut self, other_boxed: &dyn AnySpecDataType)
    where
        Self: Sized,
    {
        let other: Option<&Self> = other_boxed.downcast_ref();
        let Some(other) = other else {
            let self_ty_name = tynm::type_name::<Self>();
            panic!("Failed to downcast value into `{self_ty_name}`. Value: `{other_boxed:#?}`.");
        };
        match self {
            // Use the spec that was previously stored
            // (as opposed to previous value).
            Self::Stored => *self = other.clone(),

            // Use set value / no change on these variants
            Self::InMemory | Self::MappingFn(_) | Self::Value(_) => {}
        }
    }
}

impl<T> ValueSpecRt for ValueSpec<T>
where
    T: Clone + Debug + Serialize + Send + Sync + 'static,
{
    type ValueType = T;

    fn resolve(
        &self,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<T, ParamsResolveError> {
        ValueSpec::<T>::resolve(self, resources, value_resolution_ctx)
    }

    fn try_resolve(
        &self,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<Option<T>, ParamsResolveError> {
        ValueSpec::<T>::resolve_partial(self, resources, value_resolution_ctx)
    }
}
