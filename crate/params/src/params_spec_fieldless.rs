use std::fmt::Debug;

use peace_resource_rt::{
    resources::ts::SetUp, type_reg::untagged::BoxDataTypeDowncast, BorrowFail, Resources,
};
use serde::{Deserialize, Serialize};

use crate::{
    AnySpecDataType, AnySpecRt, MappingFnId, MappingFnReg, MappingFns, ParamsFieldless,
    ParamsResolveError, ValueResolutionCtx, ValueSpecRt,
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
///    registered from `ItemId` to `ParamsSpecFieldlessDe<T, F, U>`, where `F`
///    and `U` are derived from the `ValueSpec` provided by the user.
///
/// 2. `value_specs.yaml` is deserialized using that type registry.
///
/// 3. Each `ParamsSpecFieldlessDe<T>` is mapped into a
///    `ParamsSpecFieldless<T>`, and subsequently `AnySpecRtBoxed` to be passed
///    around in a `CmdCtx`.
///
/// 4. These `AnySpecRtBoxed`s are downcasted back to `ParamsSpecFieldless<T,
///    MFns>` when resolving values for item params and params partials.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "T: ParamsFieldless")]
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
    MappingFn {
        /// Name of the field to be mapped. `None` if this is the top level
        /// object.
        field_name: Option<String>,
        /// The name of the mapping function.
        mapping_fn_id: MappingFnId,
    },
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
    /// Returns the `ParamsSpecFieldless::MappingFn` variant with the passed in
    /// values.
    ///
    /// This is a convenience method for creating a
    /// `ParamsSpecFieldless::MappingFn` variant where the mapping function
    /// name is retrieved from `mapping_fns.name()`.
    pub fn mapping_fn<MFns>(field_name: Option<String>, mapping_fns: MFns) -> Self
    where
        MFns: MappingFns,
    {
        Self::MappingFn {
            field_name,
            mapping_fn_id: mapping_fns.id(),
        }
    }

    /// Returns the value of `T` by applying this spec to the passed in
    /// `resources`.
    pub fn resolve(
        &self,
        mapping_fn_reg: &MappingFnReg,
        resources: &Resources<peace_resource_rt::resources::ts::SetUp>,
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
            ParamsSpecFieldless::MappingFn {
                field_name,
                mapping_fn_id,
            } => resolve_t_from_mapping_fn(
                mapping_fn_reg,
                resources,
                value_resolution_ctx,
                field_name.as_deref(),
                mapping_fn_id,
            ),
        }
    }

    pub fn resolve_partial(
        &self,
        mapping_fn_reg: &MappingFnReg,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<T::Partial, ParamsResolveError> {
        match self {
            ParamsSpecFieldless::Value { value } => Ok(T::Partial::from((*value).clone())),
            ParamsSpecFieldless::Stored | ParamsSpecFieldless::InMemory => {
                match resources.try_borrow::<T>() {
                    Ok(value) => Ok(T::Partial::from((*value).clone())),
                    Err(borrow_fail) => match borrow_fail {
                        BorrowFail::ValueNotFound => Ok(T::Partial::default()),
                        BorrowFail::BorrowConflictImm | BorrowFail::BorrowConflictMut => {
                            Err(ParamsResolveError::InMemoryBorrowConflict {
                                value_resolution_ctx: value_resolution_ctx.clone(),
                            })
                        }
                    },
                }
            }
            ParamsSpecFieldless::MappingFn {
                field_name,
                mapping_fn_id,
            } => {
                let mapping_fn = mapping_fn_reg.get(mapping_fn_id).ok_or_else(|| {
                    ParamsResolveError::mapping_fn_resolve(value_resolution_ctx, mapping_fn_id)
                })?;
                let box_dt_params_opt =
                    mapping_fn.try_map(resources, value_resolution_ctx, field_name.as_deref())?;

                let t_partial = box_dt_params_opt
                    .map(|box_dt_params| {
                        BoxDataTypeDowncast::<T>::downcast_ref(&box_dt_params)
                            .cloned()
                            .ok_or_else(|| ParamsResolveError::FromMapDowncast {
                                value_resolution_ctx: value_resolution_ctx.clone(),
                                to_type_name: tynm::type_name::<T>(),
                            })
                            .map(T::Partial::from)
                    })
                    .transpose()?
                    .unwrap_or_default();

                Ok(t_partial)
            }
        }
    }
}

/// Returns a `T` by downcasting it from a `BoxDt` resolved by a mapping
/// function.
///
/// # Note
///
/// Update `ParamsSpec` as well when updating this code.
fn resolve_t_from_mapping_fn<T>(
    mapping_fn_reg: &MappingFnReg,
    resources: &Resources<SetUp>,
    value_resolution_ctx: &mut ValueResolutionCtx,
    field_name: Option<&str>,
    mapping_fn_id: &MappingFnId,
) -> Result<T, ParamsResolveError>
where
    T: ParamsFieldless<Spec = ParamsSpecFieldless<T>> + Clone + Debug + Send + Sync + 'static,
    T: ParamsFieldless,
{
    let mapping_fn = mapping_fn_reg.get(mapping_fn_id).ok_or_else(|| {
        ParamsResolveError::mapping_fn_resolve(value_resolution_ctx, mapping_fn_id)
    })?;
    let box_dt_params = mapping_fn.map(resources, value_resolution_ctx, field_name)?;

    BoxDataTypeDowncast::<T>::downcast_ref(&box_dt_params)
        .cloned()
        .ok_or_else(|| ParamsResolveError::FromMapDowncast {
            value_resolution_ctx: value_resolution_ctx.clone(),
            to_type_name: tynm::type_name::<T>(),
        })
}

impl<T> AnySpecRt for ParamsSpecFieldless<T>
where
    T: ParamsFieldless<Spec = ParamsSpecFieldless<T>>
        + Clone
        + Debug
        + Serialize
        + Send
        + Sync
        + 'static,
{
    fn is_usable(&self) -> bool {
        match self {
            Self::Stored => false,
            Self::Value { .. } | Self::InMemory | Self::MappingFn { .. } => true,
        }
    }

    fn merge(&mut self, other_boxed: &dyn AnySpecDataType)
    where
        Self: Sized,
    {
        let other: Option<&Self> = other_boxed.downcast_ref();
        let other = other.unwrap_or_else(
            #[cfg_attr(coverage_nightly, coverage(off))]
            || {
                let self_ty_name = tynm::type_name::<Self>();
                panic!(
                    "Failed to downcast value into `{self_ty_name}`. Value: `{other_boxed:#?}`."
                );
            },
        );
        match self {
            // Use the spec that was previously stored
            // (as opposed to previous value).
            Self::Stored => *self = other.clone(),

            // Use set value / no change on these variants
            Self::Value { .. } | Self::InMemory | Self::MappingFn { .. } => {}
        }
    }
}

impl<T> ValueSpecRt for ParamsSpecFieldless<T>
where
    T: ParamsFieldless<Spec = ParamsSpecFieldless<T>>
        + Clone
        + Debug
        + Serialize
        + Send
        + Sync
        + 'static,
    T::Partial: From<T>,
    T: TryFrom<T::Partial>,
{
    type ValueType = T;

    fn resolve(
        &self,
        mapping_fn_reg: &MappingFnReg,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<T, ParamsResolveError> {
        ParamsSpecFieldless::<T>::resolve(self, mapping_fn_reg, resources, value_resolution_ctx)
    }

    fn try_resolve(
        &self,
        mapping_fn_reg: &MappingFnReg,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<Option<T>, ParamsResolveError> {
        ParamsSpecFieldless::<T>::resolve_partial(
            self,
            mapping_fn_reg,
            resources,
            value_resolution_ctx,
        )
        .map(T::try_from)
        .map(Result::ok)
    }
}
