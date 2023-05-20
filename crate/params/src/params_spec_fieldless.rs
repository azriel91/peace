use std::fmt::{self, Debug};

use peace_resources::{resources::ts::SetUp, BorrowFail, Resources};
use serde::{Deserialize, Serialize};

use crate::{
    AnySpecRt, MappingFn, MappingFnImpl, ParamsFieldless, ParamsResolveError, ValueResolutionCtx,
    ValueSpecRt,
};

/// How to populate a field's value in an item spec's params.
///
/// The `MappingFn` variant's mapping function is `None` when deserialized, as
/// it is impossible to determine the underlying `F` and `U` type parameters for
/// the backing `MappingFnImpl`.
///
/// For deserialization:
///
/// 1. A `ParamsSpecsTypeReg` is constructed, and deserialization functions are
///    registered from `ItemSpecId` to `ParamsSpecFieldlessDe<T, F, U>`, where
///    `F` and `U` are derived from the `ValueSpec` provided by the user.
///
/// 2. `value_specs.yaml` is deserialized using that type registry.
///
/// 3. Each `ParamsSpecFieldlessDe<T>` is mapped into a
/// `ParamsSpecFieldless<T>`, and    subsequently `BoxDt` to be passed around in
/// a `CmdCtx`.
///
/// 4. These `BoxDt`s are downcasted back to `ParamsSpecFieldless<T>` when
/// resolving    values for item spec params and params partials.
#[derive(Clone, Serialize, Deserialize)]
#[serde(from = "crate::ParamsSpecFieldlessDe<T>")]
pub enum ParamsSpecFieldless<T>
where
    T: ParamsFieldless + Clone + Debug + Send + Sync + 'static,
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
    /// Uses the provided value.
    ///
    /// The value used is whatever is passed in to the command context
    /// builder.
    Value {
        /// The value to use.
        value: T,
    },
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
}

impl<T> ParamsSpecFieldless<T>
where
    T: ParamsFieldless + Clone + Debug + Send + Sync + 'static,
{
    pub fn from_map<F, Args>(field_name: Option<String>, f: F) -> Self
    where
        MappingFnImpl<T, F, Args>: From<(Option<String>, F)> + MappingFn<Output = T>,
    {
        let mapping_fn = MappingFnImpl::from((field_name, f));
        Self::MappingFn(Box::new(mapping_fn))
    }
}

impl<T> Debug for ParamsSpecFieldless<T>
where
    T: ParamsFieldless + Clone + Debug + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stored => f.write_str("Stored"),
            Self::Value { value } => f.debug_tuple("Value").field(value).finish(),
            Self::InMemory => f.write_str("From"),
            Self::MappingFn(mapping_fn) => f.debug_tuple("MappingFn").field(mapping_fn).finish(),
        }
    }
}

impl<T> From<T> for ParamsSpecFieldless<T>
where
    T: ParamsFieldless + Clone + Debug + Send + Sync + 'static,
{
    fn from(value: T) -> Self {
        Self::Value { value }
    }
}

impl<T> ParamsSpecFieldless<T>
where
    T: ParamsFieldless<Spec = ParamsSpecFieldless<T>> + Clone + Debug + Send + Sync + 'static,
    T::Partial: From<T>,
{
    pub fn resolve(
        &self,
        resources: &Resources<peace_resources::resources::ts::SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<T, ParamsResolveError> {
        match self {
            ParamsSpecFieldless::Value { value } => Ok(value.clone()),
            ParamsSpecFieldless::Stored | ParamsSpecFieldless::InMemory => {
                match resources.try_borrow::<T>() {
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
                }
            }
            ParamsSpecFieldless::MappingFn(mapping_fn) => {
                mapping_fn.map(resources, value_resolution_ctx)
            }
        }
    }

    pub fn resolve_partial(
        &self,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<T::Partial, ParamsResolveError> {
        match self {
            ParamsSpecFieldless::Value { value } => Ok(T::Partial::from((*value).clone())),
            ParamsSpecFieldless::Stored | ParamsSpecFieldless::InMemory => {
                match resources.try_borrow::<T>() {
                    Ok(value) => Ok(T::Partial::from((*value).clone())),
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
                }
            }
            ParamsSpecFieldless::MappingFn(mapping_fn) => mapping_fn
                .try_map(resources, value_resolution_ctx)
                .map(|t| t.map(T::Partial::from).unwrap_or_else(T::Partial::default)),
        }
    }
}

impl<T> AnySpecRt for ParamsSpecFieldless<T>
where
    T: ParamsFieldless<Spec = ParamsSpecFieldless<T>> + Clone + Debug + Send + Sync + 'static,
{
    fn is_usable(&self) -> bool {
        match self {
            Self::Stored | Self::Value { value: _ } | Self::InMemory => true,
            Self::MappingFn(mapping_fn) => mapping_fn.is_valued(),
        }
    }
}

impl<T> ValueSpecRt for ParamsSpecFieldless<T>
where
    T: ParamsFieldless<Spec = ParamsSpecFieldless<T>> + Clone + Debug + Send + Sync + 'static,
    T::Partial: From<T>,
    T: TryFrom<T::Partial>,
{
    type ValueType = T;

    fn resolve(
        &self,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<T, ParamsResolveError> {
        ParamsSpecFieldless::<T>::resolve(self, resources, value_resolution_ctx)
    }

    fn try_resolve(
        &self,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<Option<T>, ParamsResolveError> {
        ParamsSpecFieldless::<T>::resolve_partial(self, resources, value_resolution_ctx)
            .map(T::try_from)
            .map(Result::ok)
    }
}
