mod struct_params {
    use std::any::TypeId;

    use peace::params::{Params, ValueSpec};

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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
        #[allow(dead_code)]
        Named {
            /// Source / desired value for the state.
            src: String,
            /// Marker for unique parameters type.
            marker: PhantomData<Id>,
        },
        #[allow(dead_code)]
        Tuple(String),
        #[allow(dead_code)]
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
    fn spec_debug_unit() {
        assert_eq!(r#"Unit"#, format!("{:?}", EnumParamsSpec::<()>::Unit));
    }

    #[test]
    fn spec_builder_debug_unit() {
        assert_eq!(
            r#"Unit"#,
            format!("{:?}", EnumParamsSpecBuilder::<()>::Unit)
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
