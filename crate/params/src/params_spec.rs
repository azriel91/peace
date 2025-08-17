use std::fmt::Debug;

use peace_resource_rt::{
    resources::ts::SetUp, type_reg::untagged::BoxDataTypeDowncast, BorrowFail, Resources,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    AnySpecDataType, AnySpecRt, FieldWiseSpecRt, MappingFnReg, MappingFns, Params,
    ParamsResolveError, ValueResolutionCtx, ValueResolutionMode, ValueSpecRt,
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
///    registered from `ItemId` to `ParamsSpecDe<T, F, U>`, where `F` and `U`
///    are derived from the `ValueSpec` provided by the user.
///
/// 2. `value_specs.yaml` is deserialized using that type registry.
///
/// 3. Each `ParamsSpecDe<T>` is mapped into a `ValueSpec<T>`, and subsequently
///    `AnySpecRtBoxed` to be passed around in a `CmdCtx`.
///
/// 4. These `AnySpecRtBoxed`s are downcasted back to `ValueSpec<T>` when
///    resolving values for item params and params partials.
///
/// # Type Parameters
///
/// * `T`: The `Item::Params` type.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "T: Params")]
pub enum ParamsSpec<T, MFns>
where
    T: Params,
    MFns: MappingFns,
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
        /// The name of the mapping function.
        #[serde(rename = "name")]
        m_fns: MFns,
    },
    /// Resolves this value through `ValueSpec`s for each of its fields.
    ///
    /// This is like `T`, but with each field wrapped in
    /// `ParamsSpecFieldless<T>`.
    //
    // Wrap each in `ValueSpec`, but for unit / external values, fail on field wise
    // resolution, and also don't generate a builder method for field wise (even if is present in
    // the `ValueSpec` API).
    //
    // Need to decide on:
    //
    // * Every non-recursive field is annotated with `#[params(non_recursive)]`
    // * Every recursive field is annotated with `#[params(recursive)]`
    //
    // There shouldn't need to be automatic detection of non-recursive fields for stdlib types,
    // because `peace_params` should just implement `ValueSpec` for those types.
    FieldWise {
        /// The field wise spec.
        field_wise_spec: T::FieldWiseSpec,
    },
}

impl<T, MFns> From<T> for ParamsSpec<T, MFns>
where
    T: Params,
    MFns: MappingFns,
{
    fn from(value: T) -> Self {
        Self::Value { value }
    }
}

impl<T, MFns> ParamsSpec<T, MFns>
where
    T: Params<Spec = ParamsSpec<T, MFns>>,
    T::Partial: From<T>,
    MFns: MappingFns,
{
    pub fn resolve(
        &self,
        mapping_fn_reg: &MappingFnReg,
        resources: &Resources<peace_resource_rt::resources::ts::SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<T, ParamsResolveError>
    where
        T: Params,
    {
        match self {
            ParamsSpec::Value { value } => Ok(value.clone()),
            ParamsSpec::Stored | ParamsSpec::InMemory => {
                // Try resolve `T`, through the `value_resolution_ctx` first
                let params_resolved = match value_resolution_ctx.value_resolution_mode() {
                    #[cfg(feature = "item_state_example")]
                    ValueResolutionMode::Example => resources
                        .try_borrow::<peace_data::marker::Example<T>>()
                        .map(|data_marker| data_marker.0.clone()),
                    ValueResolutionMode::Clean => resources
                        .try_borrow::<peace_data::marker::Clean<T>>()
                        .map(|data_marker| data_marker.0.clone()),
                    ValueResolutionMode::Current => resources
                        .try_borrow::<peace_data::marker::Current<T>>()
                        .map(|data_marker| data_marker.0.clone()),
                    ValueResolutionMode::Goal => resources
                        .try_borrow::<peace_data::marker::Goal<T>>()
                        .map(|data_marker| data_marker.0.clone()),
                    ValueResolutionMode::ApplyDry => resources
                        .try_borrow::<peace_data::marker::ApplyDry<T>>()
                        .map(|data_marker| data_marker.0.clone()),
                }
                .and_then(|param_opt| param_opt.ok_or(BorrowFail::ValueNotFound));

                params_resolved.or_else(|_e| {
                    // Try resolve `T` again without the `value_resolution_ctx` wrapper.
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
                })
            }
            ParamsSpec::MappingFn { m_fns } => {
                resolve_t_from_mapping_fn(mapping_fn_reg, resources, value_resolution_ctx, *m_fns)
            }
            ParamsSpec::FieldWise { field_wise_spec } => {
                field_wise_spec.resolve(resources, value_resolution_ctx)
            }
        }
    }

    pub fn resolve_partial(
        &self,
        mapping_fn_reg: &MappingFnReg,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<T::Partial, ParamsResolveError> {
        match self {
            ParamsSpec::Value { value } => Ok(T::Partial::from((*value).clone())),
            ParamsSpec::Stored | ParamsSpec::InMemory => {
                // Try resolve `T`, through the `value_resolution_ctx` first
                let params_partial_resolved = match value_resolution_ctx.value_resolution_mode() {
                    #[cfg(feature = "item_state_example")]
                    ValueResolutionMode::Example => resources
                        .try_borrow::<peace_data::marker::Example<T>>()
                        .map(|data_marker| data_marker.0.clone()),
                    ValueResolutionMode::Clean => resources
                        .try_borrow::<peace_data::marker::Clean<T>>()
                        .map(|data_marker| data_marker.0.clone()),
                    ValueResolutionMode::Current => resources
                        .try_borrow::<peace_data::marker::Current<T>>()
                        .map(|data_marker| data_marker.0.clone()),
                    ValueResolutionMode::Goal => resources
                        .try_borrow::<peace_data::marker::Goal<T>>()
                        .map(|data_marker| data_marker.0.clone()),
                    ValueResolutionMode::ApplyDry => resources
                        .try_borrow::<peace_data::marker::ApplyDry<T>>()
                        .map(|data_marker| data_marker.0.clone()),
                }
                .and_then(|param_opt| param_opt.ok_or(BorrowFail::ValueNotFound));

                params_partial_resolved.map(T::Partial::from).or_else(|_e| {
                    // Try resolve `T` again without the `value_resolution_ctx` wrapper.
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
                })
            }
            ParamsSpec::MappingFn { m_fns } => {
                let mapping_fn = mapping_fn_reg
                    .get(*m_fns)
                    .ok_or_else(|| m_fns.into_params_resolve_error(value_resolution_ctx.clone()))?;
                let box_dt_params_opt = mapping_fn.try_map(resources, value_resolution_ctx)?;

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
            ParamsSpec::FieldWise { field_wise_spec } => {
                field_wise_spec.resolve_partial(resources, value_resolution_ctx)
            }
        }
    }
}

/// Returns a `T` by downcasting it from a `BoxDt` resolved by a mapping
/// function.
///
/// # Note
///
/// Update `ParamsSpecFieldless` and `ValueSpec` as well when updating this
/// code.
fn resolve_t_from_mapping_fn<T, MFns>(
    mapping_fn_reg: &MappingFnReg,
    resources: &Resources<SetUp>,
    value_resolution_ctx: &mut ValueResolutionCtx,
    m_fns: MFns,
) -> Result<T, ParamsResolveError>
where
    T: Params<Spec = ParamsSpec<T, MFns>> + Clone + Debug + Send + Sync + 'static,
    MFns: MappingFns,
{
    let mapping_fn = mapping_fn_reg
        .get(m_fns)
        .ok_or_else(|| m_fns.into_params_resolve_error(value_resolution_ctx.clone()))?;
    let box_dt_params = mapping_fn.map(resources, value_resolution_ctx)?;

    BoxDataTypeDowncast::<T>::downcast_ref(&box_dt_params)
        .cloned()
        .ok_or_else(|| ParamsResolveError::FromMapDowncast {
            value_resolution_ctx: value_resolution_ctx.clone(),
            to_type_name: tynm::type_name::<T>(),
        })
}

impl<T, MFns> AnySpecRt for ParamsSpec<T, MFns>
where
    T: Params<Spec = ParamsSpec<T, MFns>>
        + Clone
        + Debug
        + Serialize
        + DeserializeOwned
        + Send
        + Sync
        + 'static,
    MFns: MappingFns,
{
    fn is_usable(&self) -> bool {
        match self {
            Self::Stored => false,
            Self::Value { .. } | Self::InMemory | Self::MappingFn { .. } => true,
            Self::FieldWise { field_wise_spec } => field_wise_spec.is_usable(),
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

            Self::FieldWise { field_wise_spec } => {
                match other {
                    // Don't merge stored field wise specs over provided specs.
                    Self::Stored | Self::Value { .. } | Self::InMemory | Self::MappingFn { .. } => {
                    }

                    // Merge specs fieldwise.
                    Self::FieldWise {
                        field_wise_spec: field_wise_spec_other,
                    } => AnySpecRt::merge(field_wise_spec, field_wise_spec_other),
                }
            }
        }
    }
}

impl<T, MFns> ValueSpecRt for ParamsSpec<T, MFns>
where
    T: Params<Spec = ParamsSpec<T, MFns>>
        + Clone
        + Debug
        + Serialize
        + DeserializeOwned
        + Send
        + Sync
        + 'static,
    T::Partial: From<T>,
    T: TryFrom<T::Partial>,
    MFns: MappingFns,
{
    type ValueType = T;

    fn resolve(
        &self,
        mapping_fn_reg: &MappingFnReg,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<T, ParamsResolveError> {
        ParamsSpec::<T, MFns>::resolve(self, mapping_fn_reg, resources, value_resolution_ctx)
    }

    fn try_resolve(
        &self,
        mapping_fn_reg: &MappingFnReg,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<Option<T>, ParamsResolveError> {
        ParamsSpec::<T, MFns>::resolve_partial(
            self,
            mapping_fn_reg,
            resources,
            value_resolution_ctx,
        )
        .map(T::try_from)
        .map(Result::ok)
    }
}
