mod struct_params {
    use std::any::TypeId;

    use peace::params::{Params, ValueSpec};

    #[derive(Params)]
    pub struct StructParams {
        /// Source / desired value for the state.
        src: String,
    }

    super::params_tests!(
        StructParams,
        StructParamsSpec,
        StructParamsSpecBuilder,
        StructParamsPartial,
        []
    );

    #[test]
    fn spec_from_params() {
        let params = StructParams {
            src: String::from("a"),
        };

        assert!(matches!(
            StructParamsSpec::from(params),
            StructParamsSpec {
                src: ValueSpec::Value(value),
            }
            if value == "a"
        ));
    }

    #[test]
    fn spec_debug() {
        assert_eq!(
            r#"StructParamsSpec { src: Value("a") }"#,
            format!(
                "{:?}",
                StructParamsSpec {
                    src: ValueSpec::Value(String::from("a")),
                }
            )
        );
    }

    #[test]
    fn spec_builder_debug() {
        assert_eq!(
            r#"StructParamsSpecBuilder { src: Some(Value("a")) }"#,
            format!(
                "{:?}",
                StructParamsSpecBuilder {
                    src: Some(ValueSpec::Value(String::from("a"))),
                }
            )
        );
    }

    #[test]
    fn params_partial_debug() {
        assert_eq!(
            r#"StructParamsPartial { src: Some("a") }"#,
            format!(
                "{:?}",
                StructParamsPartial {
                    src: Some(String::from("a")),
                }
            )
        );
    }
}

mod struct_with_type_params {
    use std::{any::TypeId, marker::PhantomData};

    use peace::params::{Params, ValueSpec};

    #[derive(Params)]
    pub struct StructWithTypeParams<Id> {
        /// Source / desired value for the state.
        src: String,
        /// Marker for unique parameters type.
        marker: PhantomData<Id>,
    }

    super::params_tests!(
        StructWithTypeParams,
        StructWithTypeParamsSpec,
        StructWithTypeParamsSpecBuilder,
        StructWithTypeParamsPartial,
        [<()>]
    );

    #[test]
    fn spec_from_params() {
        let params = StructWithTypeParams::<()> {
            src: String::from("a"),
            marker: PhantomData,
        };

        assert!(matches!(
            StructWithTypeParamsSpec::from(params),
            StructWithTypeParamsSpec::<()> {
                src: ValueSpec::Value(value),
                marker: PhantomData,
            }
            if value == "a"
        ));
    }

    #[test]
    fn spec_debug() {
        assert_eq!(
            r#"StructWithTypeParamsSpec { src: Value("a"), marker: PhantomData<()> }"#,
            format!(
                "{:?}",
                StructWithTypeParamsSpec::<()> {
                    src: ValueSpec::Value(String::from("a")),
                    marker: PhantomData,
                }
            )
        );
    }

    #[test]
    fn spec_builder_debug() {
        assert_eq!(
            r#"StructWithTypeParamsSpecBuilder { src: Some(Value("a")), marker: PhantomData<()> }"#,
            format!(
                "{:?}",
                StructWithTypeParamsSpecBuilder::<()> {
                    src: Some(ValueSpec::Value(String::from("a"))),
                    marker: PhantomData,
                }
            )
        );
    }

    #[test]
    fn params_partial_debug() {
        assert_eq!(
            r#"StructWithTypeParamsPartial { src: Some("a"), marker: PhantomData<()> }"#,
            format!(
                "{:?}",
                StructWithTypeParamsPartial::<()> {
                    src: Some(String::from("a")),
                    marker: PhantomData,
                }
            )
        );
    }
}

mod enum_params {
    use std::{any::TypeId, marker::PhantomData};

    use peace::params::{Params, ValueSpec};

    #[derive(Params)]
    pub enum EnumParams<Id> {
        Named {
            /// Source / desired value for the state.
            src: String,
            /// Marker for unique parameters type.
            marker: PhantomData<Id>,
        },
        Tuple(String),
        TupleMarker(String, PhantomData<Id>),
        Unit,
    }

    super::params_tests!(
        EnumParams,
        EnumParamsSpec,
        EnumParamsSpecBuilder,
        EnumParamsPartial,
        [<()>]
    );

    #[test]
    fn spec_named_from_params() {
        let params = EnumParams::<()>::Named {
            src: String::from("a"),
            marker: PhantomData,
        };

        assert!(matches!(
            EnumParamsSpec::from(params),
            EnumParamsSpec::<()>::Named {
                src: ValueSpec::Value(value),
                marker: PhantomData,
            }
            if value == "a"
        ));
    }

    #[test]
    fn spec_tuple_from_params() {
        let params = EnumParams::<()>::Tuple(String::from("a"));

        assert!(matches!(
            EnumParamsSpec::from(params),
            EnumParamsSpec::<()>::Tuple(ValueSpec::Value(value))
            if value == "a"
        ));
    }

    #[test]
    fn spec_tuple_marker_from_params() {
        let params = EnumParams::<()>::TupleMarker(String::from("a"), PhantomData);

        assert!(matches!(
            EnumParamsSpec::from(params),
            EnumParamsSpec::<()>::TupleMarker(ValueSpec::Value(value), PhantomData)
            if value == "a"
        ));
    }

    #[test]
    fn spec_unit_from_params() {
        let params = EnumParams::<()>::Unit;

        assert!(matches!(
            EnumParamsSpec::from(params),
            EnumParamsSpec::<()>::Unit
        ));
    }

    #[test]
    fn spec_clone_named() {
        let spec = EnumParamsSpec::<()>::Named {
            src: ValueSpec::Value(String::from("a")),
            marker: PhantomData,
        };
        let spec_clone = spec.clone();
        drop(spec);

        assert!(matches!(
            spec_clone,
            EnumParamsSpec::<()>::Named {
                src: ValueSpec::Value(value),
                marker: PhantomData
            }
            if value == "a"
        ));
    }

    #[test]
    fn spec_clone_tuple() {
        let spec = EnumParamsSpec::<()>::Tuple(ValueSpec::Value(String::from("a")));
        let spec_clone = spec.clone();
        drop(spec);

        assert!(matches!(
            spec_clone,
            EnumParamsSpec::<()>::Tuple(ValueSpec::Value(value))
            if value == "a"
        ));
    }

    #[test]
    fn spec_clone_tuple_marker() {
        let spec =
            EnumParamsSpec::<()>::TupleMarker(ValueSpec::Value(String::from("a")), PhantomData);
        let spec_clone = spec.clone();
        drop(spec);

        assert!(matches!(
            spec_clone,
            EnumParamsSpec::<()>::TupleMarker(ValueSpec::Value(value), PhantomData)
            if value == "a"
        ));
    }

    #[test]
    fn spec_clone_unit() {
        let spec = EnumParamsSpec::<()>::Unit;
        let spec_clone = spec.clone();
        drop(spec);

        assert!(matches!(spec_clone, EnumParamsSpec::<()>::Unit));
    }

    #[test]
    fn spec_debug_named() {
        assert_eq!(
            r#"Named { src: Value("a"), marker: PhantomData<()> }"#,
            format!(
                "{:?}",
                EnumParamsSpec::<()>::Named {
                    src: ValueSpec::Value(String::from("a")),
                    marker: PhantomData,
                }
            )
        );
    }

    #[test]
    fn spec_debug_tuple() {
        assert_eq!(
            r#"Tuple(Value("a"))"#,
            format!(
                "{:?}",
                EnumParamsSpec::<()>::Tuple(ValueSpec::Value(String::from("a")))
            )
        );
    }

    #[test]
    fn spec_debug_tuple_marker() {
        assert_eq!(
            r#"TupleMarker(Value("a"), PhantomData<()>)"#,
            format!(
                "{:?}",
                EnumParamsSpec::<()>::TupleMarker(ValueSpec::Value(String::from("a")), PhantomData)
            )
        );
    }

    #[test]
    fn spec_debug_unit() {
        assert_eq!(r#"Unit"#, format!("{:?}", EnumParamsSpec::<()>::Unit));
    }

    #[test]
    fn spec_builder_clone_named() {
        let spec_builder = EnumParamsSpecBuilder::<()>::Named {
            src: Some(ValueSpec::Value(String::from("a"))),
            marker: PhantomData,
        };
        let spec_builder_clone = spec_builder.clone();
        drop(spec_builder);

        assert!(matches!(
            spec_builder_clone,
            EnumParamsSpecBuilder::<()>::Named {
                src: Some(ValueSpec::Value(value)),
                marker: PhantomData
            }
            if value == "a"
        ));
    }

    #[test]
    fn spec_builder_clone_tuple() {
        let spec_builder =
            EnumParamsSpecBuilder::<()>::Tuple(Some(ValueSpec::Value(String::from("a"))));
        let spec_builder_clone = spec_builder.clone();
        drop(spec_builder);

        assert!(matches!(
            spec_builder_clone,
            EnumParamsSpecBuilder::<()>::Tuple(Some(ValueSpec::Value(value)))
            if value == "a"
        ));
    }

    #[test]
    fn spec_builder_clone_tuple_marker() {
        let spec_builder = EnumParamsSpecBuilder::<()>::TupleMarker(
            Some(ValueSpec::Value(String::from("a"))),
            PhantomData,
        );
        let spec_builder_clone = spec_builder.clone();
        drop(spec_builder);

        assert!(matches!(
            spec_builder_clone,
            EnumParamsSpecBuilder::<()>::TupleMarker(Some(ValueSpec::Value(value)), PhantomData)
            if value == "a"
        ));
    }

    #[test]
    fn spec_builder_clone_unit() {
        let spec_builder = EnumParamsSpecBuilder::<()>::Unit;
        let spec_builder_clone = spec_builder.clone();
        drop(spec_builder);

        assert!(matches!(
            spec_builder_clone,
            EnumParamsSpecBuilder::<()>::Unit
        ));
    }

    #[test]
    fn spec_builder_debug_named() {
        assert_eq!(
            r#"Named { src: Some(Value("a")), marker: PhantomData<()> }"#,
            format!(
                "{:?}",
                EnumParamsSpecBuilder::<()>::Named {
                    src: Some(ValueSpec::Value(String::from("a"))),
                    marker: PhantomData,
                }
            )
        );
    }

    #[test]
    fn spec_builder_debug_tuple() {
        assert_eq!(
            r#"Tuple(Some(Value("a")))"#,
            format!(
                "{:?}",
                EnumParamsSpecBuilder::<()>::Tuple(Some(ValueSpec::Value(String::from("a"))))
            )
        );
    }

    #[test]
    fn spec_builder_debug_tuple_marker() {
        assert_eq!(
            r#"TupleMarker(Some(Value("a")), PhantomData<()>)"#,
            format!(
                "{:?}",
                EnumParamsSpecBuilder::<()>::TupleMarker(
                    Some(ValueSpec::Value(String::from("a"))),
                    PhantomData
                )
            )
        );
    }

    #[test]
    fn spec_builder_debug_unit() {
        assert_eq!(
            r#"Unit"#,
            format!("{:?}", EnumParamsSpecBuilder::<()>::Unit)
        );
    }

    #[test]
    fn params_partial_clone_named() {
        let params_partial = EnumParamsPartial::<()>::Named {
            src: Some(String::from("a")),
            marker: PhantomData,
        };
        let params_partial_clone = params_partial.clone();
        drop(params_partial);

        assert!(matches!(
            params_partial_clone,
            EnumParamsPartial::<()>::Named {
                src: Some(value),
                marker: PhantomData
            }
            if value == "a"
        ));
    }

    #[test]
    fn params_partial_clone_tuple() {
        let params_partial = EnumParamsPartial::<()>::Tuple(Some(String::from("a")));
        let params_partial_clone = params_partial.clone();
        drop(params_partial);

        assert!(matches!(
            params_partial_clone,
            EnumParamsPartial::<()>::Tuple(Some(value))
            if value == "a"
        ));
    }

    #[test]
    fn params_partial_clone_tuple_marker() {
        let params_partial =
            EnumParamsPartial::<()>::TupleMarker(Some(String::from("a")), PhantomData);
        let params_partial_clone = params_partial.clone();
        drop(params_partial);

        assert!(matches!(
            params_partial_clone,
            EnumParamsPartial::<()>::TupleMarker(Some(value), PhantomData)
            if value == "a"
        ));
    }

    #[test]
    fn params_partial_clone_unit() {
        let params_partial = EnumParamsPartial::<()>::Unit;
        let params_partial_clone = params_partial.clone();
        drop(params_partial);

        assert!(matches!(
            params_partial_clone,
            EnumParamsPartial::<()>::Unit
        ));
    }

    #[test]
    fn params_partial_debug_named() {
        assert_eq!(
            r#"Named { src: Some("a"), marker: PhantomData<()> }"#,
            format!(
                "{:?}",
                EnumParamsPartial::<()>::Named {
                    src: Some(String::from("a")),
                    marker: PhantomData,
                }
            )
        );
    }

    #[test]
    fn params_partial_debug_tuple() {
        assert_eq!(
            r#"Tuple(Some("a"))"#,
            format!(
                "{:?}",
                EnumParamsPartial::<()>::Tuple(Some(String::from("a")))
            )
        );
    }

    #[test]
    fn params_partial_debug_tuple_marker() {
        assert_eq!(
            r#"TupleMarker(Some("a"), PhantomData<()>)"#,
            format!(
                "{:?}",
                EnumParamsPartial::<()>::TupleMarker(Some(String::from("a")), PhantomData)
            )
        );
    }

    #[test]
    fn params_partial_debug_unit() {
        assert_eq!(r#"Unit"#, format!("{:?}", EnumParamsPartial::<()>::Unit));
    }
}

macro_rules! params_tests {
    (
        $params_ty:ident,
        $params_spec_ty:ident,
        $params_spec_builder_ty:ident,
        $params_partial_ty:ident,
        [$($generics:tt)*]
    ) => {
        #[test]
        fn params_spec_associated_type_is_params_spec() {
            assert_eq!(
                TypeId::of::<<$params_ty $($generics)* as Params>::Spec>(),
                TypeId::of::<$params_spec_ty $($generics)*>()
            );
        }

        #[test]
        fn params_spec_builder_associated_type_is_params_spec_builder() {
            assert_eq!(
                TypeId::of::<<$params_ty $($generics)* as Params>::SpecBuilder>(),
                TypeId::of::<$params_spec_builder_ty $($generics)*>()
            );
        }

        #[test]
        fn params_partial_associated_type_is_params_partial() {
            assert_eq!(
                TypeId::of::<<$params_ty $($generics)* as Params>::Partial>(),
                TypeId::of::<$params_partial_ty $($generics)*>()
            );
        }
    };
}

pub(crate) use params_tests;
