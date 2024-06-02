use std::{
    fmt::{self, Debug},
    marker::PhantomData,
};

use peace_data::marker::{ApplyDry, Clean, Current, Goal};
use peace_resources_rt::{resources::ts::SetUp, BorrowFail, Resources};
use serde::{Deserialize, Serialize, Serializer};

use crate::{
    FromFunc, Func, MappingFn, ParamsResolveError, ValueResolutionCtx, ValueResolutionMode,
};

/// Wrapper around a mapping function so that it can be serialized.
#[derive(Clone, Serialize, Deserialize)]
pub struct MappingFnImpl<T, F, Args> {
    /// This field's name within its parent struct.
    ///
    /// `None` if this is the top level value type.
    field_name: Option<String>,
    #[serde(
        default = "MappingFnImpl::<T, F, Args>::fn_map_none",
        skip_deserializing,
        serialize_with = "MappingFnImpl::<T, F, Args>::fn_map_serialize"
    )]
    fn_map: Option<F>,
    /// Marker.
    marker: PhantomData<(T, Args)>,
}

impl<T, F, Args> Debug for MappingFnImpl<T, F, Args>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MappingFnImpl")
            .field("field_name", &self.field_name)
            .field("fn_map", &Self::fn_map_stringify(&self.fn_map))
            .field("marker", &self.marker)
            .finish()
    }
}

impl<T, F, Args> MappingFnImpl<T, F, Args> {
    fn fn_map_serialize<S>(fn_map: &Option<F>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&Self::fn_map_stringify(fn_map))
    }

    fn fn_map_stringify(fn_map: &Option<F>) -> String {
        match fn_map {
            Some(_) => {
                let args = {
                    let args_type_name = tynm::type_name::<Args>();
                    let mut buffer = String::with_capacity(args_type_name.len() + 32);
                    args_type_name.chars().fold(0, |mut nesting_level, c| {
                        buffer.push(c);

                        match c {
                            '(' => {
                                nesting_level += 1;
                                if nesting_level == 1 {
                                    buffer.push('&');
                                }
                            }
                            ')' => nesting_level -= 1,
                            ' ' => {
                                if nesting_level == 1 {
                                    buffer.push('&');
                                }
                            }
                            _ => (),
                        }

                        nesting_level
                    });

                    buffer
                };

                format!(
                    "Some(Fn{args} -> Option<{t}>)",
                    t = tynm::type_name::<T>(),
                    args = args,
                )
            }
            None => String::from("None"),
        }
    }

    fn fn_map_none() -> Option<F> {
        None
    }
}

macro_rules! impl_mapping_fn_impl {
    ($($Arg:ident $var:ident),+) => {
        // impl<T, F, A0> MappingFnImpl<T, F, (A0,)>
        impl<T, F, $($Arg,)+> MappingFnImpl<T, F, ($($Arg,)+)>
        where
            T: Clone + Debug + Send + Sync + 'static,
            F: Fn($(&$Arg,)+) -> Option<T> + Clone + Send + Sync + 'static,
            $($Arg: Clone + Debug + Send + Sync + 'static,)+
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
                let fn_map = self.fn_map.as_ref().unwrap_or_else(
                    #[cfg_attr(coverage_nightly, coverage(off))]
                    || {
                        panic!("`MappingFnImpl::map` called when `fn_map` is `None`\
                            {for_field_name}.\n\
                            This is a bug in the Peace framework.\n\
                            \n\
                            Type parameters are:\n\
                            \n\
                            * `T`: {t}\n\
                            * `Args`: ({Args})\n\
                            ",
                            for_field_name = self.field_name
                                .as_ref()
                                .map(|field_name| format!(" for field: `{field_name}`"))
                                .unwrap_or("".to_string()),
                            t = tynm::type_name::<T>(),
                            Args = tynm::type_name::<($($Arg,)+)>(),
                        );
                    });

                // We have to duplicate code because the return type from
                // `resources.try_borrow` is different per branch.
                match value_resolution_ctx.value_resolution_mode() {
                    ValueResolutionMode::ApplyDry => {
                        $(arg_resolve!(resources, value_resolution_ctx, ApplyDry, $var, $Arg);)+

                        fn_map($(&$var,)+).ok_or(ParamsResolveError::FromMap {
                            value_resolution_ctx: value_resolution_ctx.clone(),
                            from_type_name: tynm::type_name::<($($Arg,)+)>(),
                        })
                    }
                    ValueResolutionMode::Current => {
                        $(arg_resolve!(resources, value_resolution_ctx, Current, $var, $Arg);)+

                        fn_map($(&$var,)+).ok_or(ParamsResolveError::FromMap {
                            value_resolution_ctx: value_resolution_ctx.clone(),
                            from_type_name: tynm::type_name::<($($Arg,)+)>(),
                        })
                    }
                    ValueResolutionMode::Goal => {
                        $(arg_resolve!(resources, value_resolution_ctx, Goal, $var, $Arg);)+

                        fn_map($(&$var,)+).ok_or(ParamsResolveError::FromMap {
                            value_resolution_ctx: value_resolution_ctx.clone(),
                            from_type_name: tynm::type_name::<($($Arg,)+)>(),
                        })
                    }
                    ValueResolutionMode::Clean => {
                        $(arg_resolve!(resources, value_resolution_ctx, Clean, $var, $Arg);)+

                        fn_map($(&$var,)+).ok_or(ParamsResolveError::FromMap {
                            value_resolution_ctx: value_resolution_ctx.clone(),
                            from_type_name: tynm::type_name::<($($Arg,)+)>(),
                        })
                    }
                }
            }

            pub fn try_map(
                &self,
                resources: &Resources<SetUp>,
                value_resolution_ctx: &mut ValueResolutionCtx,
            ) -> Result<Option<T>, ParamsResolveError> {
                let fn_map = self.fn_map.as_ref().unwrap_or_else(
                    #[cfg_attr(coverage_nightly, coverage(off))]
                    || {
                        panic!("`MappingFnImpl::try_map` called when `fn_map` is `None`\
                            {for_field_name}.\n\
                            This is a bug in the Peace framework.\n\
                            \n\
                            Type parameters are:\n\
                            \n\
                            * `T`: {t}\n\
                            * `Args`: ({Args})\n\
                            ",
                            for_field_name = self.field_name
                                .as_ref()
                                .map(|field_name| format!(" for field: `{field_name}`"))
                                .unwrap_or("".to_string()),
                            t = tynm::type_name::<T>(),
                            Args = tynm::type_name::<($($Arg,)+)>(),
                        );
                    });

                // We have to duplicate code because the return type from
                // `resources.try_borrow` is different per branch.
                match value_resolution_ctx.value_resolution_mode() {
                    ValueResolutionMode::ApplyDry => {
                        $(try_arg_resolve!(resources, value_resolution_ctx, ApplyDry, $var, $Arg);)+

                        Ok(fn_map($(&$var,)+))
                    }
                    ValueResolutionMode::Current => {
                        $(try_arg_resolve!(resources, value_resolution_ctx, Current, $var, $Arg);)+

                        Ok(fn_map($(&$var,)+))
                    }
                    ValueResolutionMode::Goal => {
                        $(try_arg_resolve!(resources, value_resolution_ctx, Goal, $var, $Arg);)+

                        Ok(fn_map($(&$var,)+))
                    }
                    ValueResolutionMode::Clean => {
                        $(try_arg_resolve!(resources, value_resolution_ctx, Clean, $var, $Arg);)+

                        Ok(fn_map($(&$var,)+))
                    }
                }
            }
        }

        impl<T, F, $($Arg,)+> FromFunc<F> for MappingFnImpl<T, F, ($($Arg,)+)>
        where
            T: Clone + Debug + Send + Sync + 'static,
            // Ideally we can do:
            //
            // ```rust
            // F: Fn<($($Arg,)+), Output = Option<T>>
            // ```
            //
            // But this is pending <rust-lang/rust#29625>
            F: Func<Option<T>, ($($Arg,)+)>
                + Fn($(&$Arg,)+) -> Option<T>
                + Clone + Send + Sync + 'static,
            $($Arg: Clone + Debug + Send + Sync + 'static,)+
        {
            fn from_func(field_name: Option<String>, f: F) -> Self {
                Self::new(field_name, f)
            }
        }

        impl<T, F, $($Arg,)+> From<(Option<String>, F)> for MappingFnImpl<T, F, ($($Arg,)+)>
        where
            T: Clone + Debug + Send + Sync + 'static,
            F: Fn($(&$Arg,)+) -> Option<T> + Clone + Send + Sync + 'static,
            $($Arg: Clone + Debug + Send + Sync + 'static,)+
        {
            fn from((field_name, f): (Option<String>, F)) -> Self {
                Self::new(field_name, f)
            }
        }

        impl<T, F, $($Arg,)+> MappingFn for MappingFnImpl<T, F, ($($Arg,)+)>
        where
            T: Clone + Debug + Send + Sync + 'static,
            F: Fn($(&$Arg,)+) -> Option<T> + Clone + Send + Sync + 'static,
            $($Arg: Clone + Debug + Send + Sync + 'static,)+
        {
            type Output = T;

            fn map(
                &self,
                resources: &Resources<SetUp>,
                value_resolution_ctx: &mut ValueResolutionCtx,
            ) -> Result<<Self as MappingFn>::Output, ParamsResolveError> {
                MappingFnImpl::<T, F, ($($Arg,)+)>::map(self, resources, value_resolution_ctx)
            }

            fn try_map(
                &self,
                resources: &Resources<SetUp>,
                value_resolution_ctx: &mut ValueResolutionCtx,
            ) -> Result<Option<<Self as MappingFn>::Output>, ParamsResolveError> {
                MappingFnImpl::<T, F, ($($Arg,)+)>::try_map(self, resources, value_resolution_ctx)
            }

            fn is_valued(&self) -> bool {
                self.fn_map.is_some()
            }
        }
    };
}

#[derive(Debug)]
enum BorrowedData<Marked, T> {
    Marked(Marked),
    Direct(T),
}

macro_rules! arg_resolve {
    (
        $resources:ident,
        $value_resolution_ctx:ident,
        $value_resolution_mode:ident,
        $arg:ident,
        $Arg:ident
    ) => {
        // Prioritize data marker wrapper over direct data borrow.
        let borrow_marked_data_result = $resources.try_borrow::<$value_resolution_mode<$Arg>>();
        let borrow_direct = $resources.try_borrow::<$Arg>();

        let borrowed_data = match borrow_marked_data_result {
            Ok(borrow_marked_data) => BorrowedData::Marked(borrow_marked_data),
            Err(borrow_fail) => match borrow_fail {
                // Either:
                //
                // * `A0` in the function is incorrect, so `Current<A0>` is not registered by any
                //   item, or
                // * There is a bug in Peace.
                BorrowFail::ValueNotFound => match borrow_direct {
                    Ok(arg) => BorrowedData::Direct(arg),
                    Err(borrow_fail) => match borrow_fail {
                        // Either:
                        //
                        // * `A0` in the function is incorrect, so `Current<A0>` is not registered
                        //   by any item, or
                        // * There is a bug in Peace.
                        BorrowFail::ValueNotFound => {
                            return Err(ParamsResolveError::FromMap {
                                value_resolution_ctx: $value_resolution_ctx.clone(),
                                from_type_name: tynm::type_name::<$Arg>(),
                            });
                        }
                        BorrowFail::BorrowConflictImm | BorrowFail::BorrowConflictMut => {
                            return Err(ParamsResolveError::FromMapBorrowConflict {
                                value_resolution_ctx: $value_resolution_ctx.clone(),
                                from_type_name: tynm::type_name::<$Arg>(),
                            });
                        }
                    },
                },
                BorrowFail::BorrowConflictImm | BorrowFail::BorrowConflictMut => {
                    return Err(ParamsResolveError::FromMapBorrowConflict {
                        value_resolution_ctx: $value_resolution_ctx.clone(),
                        from_type_name: tynm::type_name::<$Arg>(),
                    });
                }
            },
        };
        let $arg = match &borrowed_data {
            BorrowedData::Marked(marked_data) => match marked_data.as_ref() {
                Some(data) => data,
                None => {
                    return Err(ParamsResolveError::FromMap {
                        value_resolution_ctx: $value_resolution_ctx.clone(),
                        from_type_name: tynm::type_name::<$Arg>(),
                    });
                }
            },
            BorrowedData::Direct(data) => data,
        };
    };
}

macro_rules! try_arg_resolve {
    (
        $resources:ident,
        $value_resolution_ctx:ident,
        $value_resolution_mode:ident,
        $arg:ident,
        $Arg:ident
    ) => {
        // Prioritize data marker wrapper over direct data borrow.
        let borrow_marked_data_result = $resources.try_borrow::<$value_resolution_mode<$Arg>>();
        let borrow_direct = $resources.try_borrow::<$Arg>();

        let borrowed_data = match borrow_marked_data_result {
            Ok(borrow_marked_data) => BorrowedData::Marked(borrow_marked_data),
            Err(borrow_fail) => match borrow_fail {
                // Either:
                //
                // * `A0` in the function is incorrect, so `Current<A0>` is not registered by any
                //   item, or
                // * There is a bug in Peace.
                BorrowFail::ValueNotFound => match borrow_direct {
                    Ok(arg) => BorrowedData::Direct(arg),
                    Err(borrow_fail) => match borrow_fail {
                        // Either:
                        //
                        // * `A0` in the function is incorrect, so `Current<A0>` is not registered
                        //   by any item, or
                        // * There is a bug in Peace.
                        BorrowFail::ValueNotFound => return Ok(None),
                        BorrowFail::BorrowConflictImm | BorrowFail::BorrowConflictMut => {
                            return Err(ParamsResolveError::FromMapBorrowConflict {
                                value_resolution_ctx: $value_resolution_ctx.clone(),
                                from_type_name: tynm::type_name::<$Arg>(),
                            });
                        }
                    },
                },
                BorrowFail::BorrowConflictImm | BorrowFail::BorrowConflictMut => {
                    return Err(ParamsResolveError::FromMapBorrowConflict {
                        value_resolution_ctx: $value_resolution_ctx.clone(),
                        from_type_name: tynm::type_name::<$Arg>(),
                    });
                }
            },
        };
        let $arg = match &borrowed_data {
            BorrowedData::Marked(marked_data) => match marked_data.as_ref() {
                Some(data) => data,
                None => return Ok(None),
            },
            BorrowedData::Direct(data) => &data,
        };
    };
}

// We can add more if we need to support more args.
//
// There is a compile time / Rust analyzer startup cost to it, so it's better to
// not generate more than we need.
impl_mapping_fn_impl!(A0 a0);
impl_mapping_fn_impl!(A0 a0, A1 a1);
impl_mapping_fn_impl!(A0 a0, A1 a1, A2 a2);
impl_mapping_fn_impl!(A0 a0, A1 a1, A2 a2, A3 a3);
impl_mapping_fn_impl!(A0 a0, A1 a1, A2 a2, A3 a3, A4 a4);
