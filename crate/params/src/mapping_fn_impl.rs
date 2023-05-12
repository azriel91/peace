use std::{
    fmt::{self, Debug},
    marker::PhantomData,
};

use peace_resources::{resources::ts::SetUp, BorrowFail, Resources};
use serde::{Deserialize, Serialize, Serializer};

use crate::{FieldNameAndType, MappingFn, ParamsResolveError, ValueResolutionCtx};

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
                if let Some(field_name) = self.field_name.as_deref() {
                    value_resolution_ctx.push(FieldNameAndType::new(
                        field_name.to_string(),
                        tynm::type_name::<T>().to_string(),
                    ));
                }
                let Some(fn_map) = self.fn_map.as_ref() else {
                    panic!("`MappingFnImpl::map` called when `fn_map` is `None`.\n\
                        This is a bug in the Peace framework.\n\
                        \n\
                        Type parameters are:\n\
                        \n\
                        * `T`: {t}\n\
                        * `Args`: ({Args})\n\
                        ",
                        t = tynm::type_name::<T>(),
                        Args = tynm::type_name::<($($Arg,)+)>(),
                        );
                };

                $(
                    let $var = resources.try_borrow::<$Arg>();
                    let $var = match $var {
                        Ok($var) => $var,
                        Err(borrow_fail) => match borrow_fail {
                            BorrowFail::ValueNotFound => {
                                return Err(ParamsResolveError::FromMap {
                                    value_resolution_ctx: value_resolution_ctx.clone(),
                                    from_type_name: tynm::type_name::<$Arg>(),
                                });
                            },
                            BorrowFail::BorrowConflictImm | BorrowFail::BorrowConflictMut => {
                                return Err(ParamsResolveError::FromMapBorrowConflict {
                                    value_resolution_ctx: value_resolution_ctx.clone(),
                                    from_type_name: tynm::type_name::<$Arg>(),
                                });
                            }
                        },
                    };
                )+

                fn_map($(&$var,)+).ok_or(ParamsResolveError::FromMap {
                    value_resolution_ctx: value_resolution_ctx.clone(),
                    from_type_name: tynm::type_name::<($($Arg,)+)>(),
                })

            }

            pub fn try_map(
                &self,
                resources: &Resources<SetUp>,
                value_resolution_ctx: &mut ValueResolutionCtx,
            ) -> Result<Option<T>, ParamsResolveError> {
                if let Some(field_name) = self.field_name.as_deref() {
                    value_resolution_ctx.push(FieldNameAndType::new(
                        field_name.to_string(),
                        tynm::type_name::<T>().to_string(),
                    ));
                }
                let Some(fn_map) = self.fn_map.as_ref() else {
                    panic!("`MappingFnImpl::try_map` called when `fn_map` is `None`.\n\
                        This is a bug in the Peace framework.\n\
                        \n\
                        Type parameters are:\n\
                        \n\
                        * `T`: {t}\n\
                        * `Args`: ({Args})\n\
                        ",
                        t = tynm::type_name::<T>(),
                        Args = tynm::type_name::<($($Arg,)+)>(),
                        );
                };

                $(
                    let $var = resources.try_borrow::<$Arg>();
                    let $var = match $var {
                        Ok($var) => $var,
                        Err(borrow_fail) => match borrow_fail {
                            BorrowFail::ValueNotFound => return Ok(None),
                            BorrowFail::BorrowConflictImm | BorrowFail::BorrowConflictMut => {
                                return Err(ParamsResolveError::FromMapBorrowConflict {
                                    value_resolution_ctx: value_resolution_ctx.clone(),
                                    from_type_name: tynm::type_name::<$Arg>(),
                                });
                            }
                        },
                    };
                )+

                Ok(fn_map($(&$var,)+))
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
        }
    };
}

use impl_mapping_fn_impl;

// We can add more if we need to support more args.
//
// There is a compile time / Rust analyzer startup cost to it, so it's better to
// not generate more than we need.
impl_mapping_fn_impl!(A0 a0);
impl_mapping_fn_impl!(A0 a0, A1 a1);
impl_mapping_fn_impl!(A0 a0, A1 a1, A2 a2);
impl_mapping_fn_impl!(A0 a0, A1 a1, A2 a2, A3 a3);
impl_mapping_fn_impl!(A0 a0, A1 a1, A2 a2, A3 a3, A4 a4);
