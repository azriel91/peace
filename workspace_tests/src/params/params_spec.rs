use peace::{
    enum_iterator::Sequence,
    item_model::item_id,
    params::{
        AnySpecRt, AnySpecRtBoxed, FieldNameAndType, FieldWiseSpecRt, FromFunc, MappingFn,
        MappingFnId, MappingFnImpl, MappingFnReg, MappingFns, Params, ParamsResolveError,
        ParamsSpec, ValueResolutionCtx, ValueResolutionMode, ValueSpec, ValueSpecRt,
    },
    resource_rt::{resources::ts::SetUp, Resources},
};
use serde::{Deserialize, Serialize};

use crate::{
    mock_item::{MockSrc, MockSrcFieldWise},
    vec_copy_item::{VecA, VecAFieldWise},
};

#[test]
fn clone() {
    let _params_spec = ParamsSpec::<MockSrc>::Value { value: MockSrc(1) }.clone();
}

#[test]
fn debug() {
    assert_eq!("Stored", format!("{:?}", ParamsSpec::<MockSrc>::Stored));
    assert_eq!(
        "Value { value: MockSrc(1) }",
        format!("{:?}", ParamsSpec::<MockSrc>::Value { value: MockSrc(1) })
    );
    assert_eq!("InMemory", format!("{:?}", ParamsSpec::<MockSrc>::InMemory));
    assert_eq!(
        "MappingFn { \
            field_name: Some(\"field\"), \
            mapping_fn_id: MappingFnId(\"MockSrcFromU8\") \
        }",
        format!(
            "{:?}",
            ParamsSpec::<MockSrc>::mapping_fn(
                Some(String::from("field")),
                TestMappingFns::MockSrcFromU8
            )
        )
    );
    assert_eq!(
        "FieldWise { field_wise_spec: MockSrcFieldWise(Stored) }",
        format!("{:?}", <MockSrc as Params>::field_wise_spec().build())
    );
}

#[test]
fn serialize_stored() -> Result<(), serde_yaml::Error> {
    let vec_a_spec: <VecA as Params>::Spec = <VecA as Params>::Spec::Stored;
    assert_eq!(
        r#"Stored
"#,
        serde_yaml::to_string(&vec_a_spec)?,
    );

    Ok(())
}

#[test]
fn serialize_value() -> Result<(), serde_yaml::Error> {
    let vec_a_spec: <VecA as Params>::Spec = VecA(vec![1u8]).into();
    assert_eq!(
        r#"!Value
value:
- 1
"#,
        serde_yaml::to_string(&vec_a_spec)?,
    );

    Ok(())
}

#[test]
fn serialize_in_memory() -> Result<(), serde_yaml::Error> {
    let vec_a_spec: <VecA as Params>::Spec = <VecA as Params>::Spec::InMemory;
    assert_eq!(
        r#"InMemory
"#,
        serde_yaml::to_string(&vec_a_spec)?,
    );

    Ok(())
}

#[test]
fn serialize_mapping_fn() -> Result<(), serde_yaml::Error> {
    let vec_a_spec: <VecA as Params>::Spec =
        ParamsSpec::<VecA>::mapping_fn(None, TestMappingFns::MockSrcFromU8);
    assert_eq!(
        r#"!MappingFn
field_name: null
mapping_fn_id: MockSrcFromU8
"#,
        serde_yaml::to_string(&vec_a_spec)?,
    );

    Ok(())
}

#[test]
fn serialize_field_wise_stored() -> Result<(), serde_yaml::Error> {
    let vec_a_spec: <VecA as Params>::Spec = VecA::field_wise_spec().build();
    assert_eq!(
        r#"!FieldWise
field_wise_spec: Stored
"#,
        serde_yaml::to_string(&vec_a_spec)?,
    );

    Ok(())
}

#[test]
fn serialize_field_wise_value() -> Result<(), serde_yaml::Error> {
    let vec_a_spec: <VecA as Params>::Spec = VecA::field_wise_spec().with_0(vec![1u8]).build();
    assert_eq!(
        r#"!FieldWise
field_wise_spec: !Value
  value:
  - 1
"#,
        serde_yaml::to_string(&vec_a_spec)?,
    );

    Ok(())
}

#[test]
fn serialize_field_wise_in_memory() -> Result<(), serde_yaml::Error> {
    let vec_a_spec: <VecA as Params>::Spec = VecA::field_wise_spec().with_0_in_memory().build();
    assert_eq!(
        r#"!FieldWise
field_wise_spec: InMemory
"#,
        serde_yaml::to_string(&vec_a_spec)?,
    );

    Ok(())
}

#[test]
fn serialize_field_wise_mapping_fn() -> Result<(), serde_yaml::Error> {
    let vec_a_spec: <VecA as Params>::Spec = VecA::field_wise_spec()
        .with_0_from_mapping_fn(TestMappingFns::VecU8FromBoolAndU16)
        .build();
    assert_eq!(
        r#"!FieldWise
field_wise_spec: !MappingFn
  field_name: _0
  mapping_fn_id: VecU8FromBoolAndU16
"#,
        serde_yaml::to_string(&vec_a_spec)?,
    );

    Ok(())
}

#[test]
fn deserialize_stored() -> Result<(), serde_yaml::Error> {
    assert!(matches!(
        serde_yaml::from_str(
            r#"Stored
        "#
        )?,
        ParamsSpec::<VecA>::Stored
    ));

    Ok(())
}

#[test]
fn deserialize_value() -> Result<(), serde_yaml::Error> {
    assert!(matches!(
        serde_yaml::from_str(
            r#"!Value
value:
- 1
"#
        )?,
        ParamsSpec::<VecA>::Value {
            value: VecA(vec_u8)
        }
        if vec_u8 == [1u8]
    ));

    Ok(())
}

#[test]
fn deserialize_in_memory() -> Result<(), serde_yaml::Error> {
    let deserialized = serde_yaml::from_str(
        r#"InMemory
"#,
    )?;

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(&deserialized, ParamsSpec::<VecA>::InMemory),
                "was {deserialized:?}"
            );
        }
    })();

    Ok(())
}

#[test]
fn deserialize_from_mapping_fn() -> Result<(), serde_yaml::Error> {
    let deserialized = serde_yaml::from_str(
        r#"!MappingFn
field_name: serialized_field_name
mapping_fn_id: VecU8FromBoolAndU16
"#,
    )?;

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &deserialized,
                    ParamsSpec::<VecA>::MappingFn {
                        field_name: Some(field_name),
                        mapping_fn_id,
                    }
                    if field_name == "serialized_field_name" &&
                    mapping_fn_id == &TestMappingFns::VecU8FromBoolAndU16.id()
                ),
                "was {deserialized:?}"
            );
        }
    })();

    Ok(())
}

#[test]
fn deserialize_field_wise_value() -> Result<(), Box<dyn std::error::Error>> {
    let resources = Resources::<SetUp>::from(Resources::new());
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::ApplyDry,
        item_id!("deserialize_field_wise"),
        tynm::type_name::<VecA>(),
    );
    let mapping_fn_reg = MappingFnReg::new();

    let deserialized = serde_yaml::from_str(
        r#"!FieldWise
field_wise_spec: !Value
  value:
  - 1
"#,
    )?;

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &deserialized,
                    ParamsSpec::<VecA>::FieldWise {
                        field_wise_spec: field_wise_spec @
                            VecAFieldWise(ValueSpec::<Vec<u8>>::Value {
                                value,
                            })
                    }
                    if value == &[1u8]
                    && FieldWiseSpecRt::resolve(
                            field_wise_spec,
                            &mapping_fn_reg,
                            &resources,
                            &mut value_resolution_ctx
                        )
                        .expect("expected value to be resolved.")
                    == VecA(vec![1u8])
                ),
                "was {deserialized:?}"
            );
        }
    })();

    Ok(())
}

#[test]
fn deserialize_field_wise_in_memory() -> Result<(), serde_yaml::Error> {
    let deserialized = serde_yaml::from_str(
        r#"!FieldWise
field_wise_spec: InMemory
"#,
    )?;

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &deserialized,
                    ParamsSpec::<VecA>::FieldWise {
                        field_wise_spec: VecAFieldWise(ValueSpec::<Vec<u8>>::InMemory)
                    }
                ),
                "was {deserialized:?}"
            );
        }
    })();

    Ok(())
}

#[test]
fn deserialize_field_wise_mapping_fn() -> Result<(), serde_yaml::Error> {
    let deserialized = serde_yaml::from_str(
        r#"!FieldWise
field_wise_spec: !MappingFn
  field_name: serialized_field_name
  mapping_fn_id: VecU8FromBoolAndU16
"#,
    )?;

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &deserialized,
                    ParamsSpec::<VecA>::FieldWise {
                        field_wise_spec: VecAFieldWise(ValueSpec::<Vec<u8>>::MappingFn {
                            field_name: Some(field_name),
                            mapping_fn_id,
                        })
                    }
                    if field_name == "serialized_field_name" &&
                    mapping_fn_id == &TestMappingFns::VecU8FromBoolAndU16.id()
                ),
                "was {deserialized:?}"
            );
        }
    })();

    Ok(())
}

#[test]
fn is_usable_returns_false_for_stored() {
    assert!(!ParamsSpec::<VecA>::Stored.is_usable());
}

#[test]
fn is_usable_returns_true_for_value_and_in_memory() {
    assert!(ParamsSpec::<VecA>::Value {
        value: VecA::default()
    }
    .is_usable());
    assert!(ParamsSpec::<VecA>::InMemory.is_usable());
}

#[test]
fn is_usable_returns_true_when_mapping_fn_is_some() {
    assert!(ParamsSpec::<VecA>::MappingFn {
        field_name: None,
        mapping_fn_id: TestMappingFns::VecU8FromBoolAndU16.id()
    }
    .is_usable());
}

#[test]
fn is_usable_returns_true_for_deserialized_mapping_fn() -> Result<(), serde_yaml::Error> {
    let params_spec: ParamsSpec<VecA> = serde_yaml::from_str(
        r#"!MappingFn
field_name: null
mapping_fn_id: VecU8FromBoolAndU16
"#,
    )?;

    assert!(params_spec.is_usable());
    Ok(())
}

#[test]
fn is_usable_returns_true_when_field_wise_is_usable() -> Result<(), serde_yaml::Error> {
    let params_spec: ParamsSpec<VecA> = serde_yaml::from_str(
        r#"!FieldWise
field_wise_spec: InMemory
"#,
    )?;

    assert!(params_spec.is_usable());
    Ok(())
}

#[test]
fn is_usable_returns_true_when_field_wise_with_mapping_fn_is_deserialized(
) -> Result<(), serde_yaml::Error> {
    let params_spec: ParamsSpec<VecA> = serde_yaml::from_str(
        r#"!FieldWise
field_wise_spec: !MappingFn
  field_name: field_name
  mapping_fn_id: VecU8FromBoolAndU16
"#,
    )?;

    assert!(params_spec.is_usable());
    Ok(())
}

#[test]
fn is_usable_returns_false_when_field_wise_contains_stored() -> Result<(), serde_yaml::Error> {
    let params_spec = ParamsSpec::<VecA>::FieldWise {
        field_wise_spec: VecAFieldWise(ValueSpec::<Vec<u8>>::Stored),
    };

    assert!(!params_spec.is_usable());
    Ok(())
}

#[test]
fn resolve_stored_param() -> Result<(), ParamsResolveError> {
    let mapping_fn_reg = MappingFnReg::new();
    let resources = {
        let mut resources = Resources::new();
        resources.insert(MockSrc(1));
        Resources::<SetUp>::from(resources)
    };
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("resolve_stored_param"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = ParamsSpec::<MockSrc>::Stored;

    let mock_src = ValueSpecRt::resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    )?;

    assert_eq!(MockSrc(1), mock_src);
    Ok(())
}

#[test]
fn resolve_in_memory() -> Result<(), ParamsResolveError> {
    let mapping_fn_reg = MappingFnReg::new();
    let resources = {
        let mut resources = Resources::new();
        resources.insert(MockSrc(1));
        Resources::<SetUp>::from(resources)
    };
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("resolve_in_memory"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = ParamsSpec::<MockSrc>::InMemory;

    let mock_src = ValueSpecRt::resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    )?;

    assert_eq!(MockSrc(1), mock_src);
    Ok(())
}

#[test]
fn resolve_in_memory_returns_err_when_not_found() -> Result<(), ParamsResolveError> {
    let mapping_fn_reg = MappingFnReg::new();
    let resources = Resources::<SetUp>::from(Resources::new());
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("resolve_in_memory_returns_err_when_not_found"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = ParamsSpec::<MockSrc>::InMemory;

    let mock_src_result = ValueSpecRt::resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    );

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &mock_src_result,
                    Err(ParamsResolveError::InMemory { value_resolution_ctx })
                    if value_resolution_ctx.value_resolution_mode() == ValueResolutionMode::Current
                    && value_resolution_ctx.item_id()
                        == &item_id!("resolve_in_memory_returns_err_when_not_found")
                    && value_resolution_ctx.params_type_name() == "MockSrc"
                    && value_resolution_ctx.resolution_chain().is_empty()
                ),
                "expected `mock_src_result` to be `Err(ParamsResolveError::InMemory {{ .. }})`\n\
                but was `{mock_src_result:?}`"
            );
        }
    })();
    Ok(())
}

#[test]
fn resolve_in_memory_returns_err_when_mutably_borrowed() -> Result<(), ParamsResolveError> {
    let mapping_fn_reg = MappingFnReg::new();
    let resources = {
        let mut resources = Resources::new();
        resources.insert(MockSrc(1));
        Resources::<SetUp>::from(resources)
    };
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("resolve_in_memory_returns_err_when_mutably_borrowed"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = ParamsSpec::<MockSrc>::InMemory;

    let _mock_src_mut_borrowed = resources.borrow_mut::<MockSrc>();
    let mock_src_result = ValueSpecRt::resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    );

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &mock_src_result,
                    Err(ParamsResolveError::InMemoryBorrowConflict { value_resolution_ctx })
                    if value_resolution_ctx.value_resolution_mode() == ValueResolutionMode::Current
                    && value_resolution_ctx.item_id()
                        == &item_id!("resolve_in_memory_returns_err_when_mutably_borrowed")
                    && value_resolution_ctx.params_type_name() == "MockSrc"
                    && value_resolution_ctx.resolution_chain().is_empty()
                ),
                "expected `mock_src_result` to be \
                `Err(ParamsResolveError::InMemoryBorrowConflict {{ .. }})`\n\
                with `resolution_chain`: `[]`,\n\
                but was `{mock_src_result:?}`"
            );
        }
    })();
    Ok(())
}

#[test]
fn resolve_value() -> Result<(), ParamsResolveError> {
    let mapping_fn_reg = MappingFnReg::new();
    let resources = Resources::<SetUp>::from(Resources::new());
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("resolve_value"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = ParamsSpec::<MockSrc>::Value { value: MockSrc(1) };

    let mock_src = ValueSpecRt::resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    )?;

    assert_eq!(MockSrc(1), mock_src);
    Ok(())
}

#[test]
fn resolve_mapping_fn() -> Result<(), ParamsResolveError> {
    let test_mapping_fn = TestMappingFns::MockSrcFromU8;
    let mut mapping_fn_reg = MappingFnReg::new();
    mapping_fn_reg.insert(test_mapping_fn.id(), test_mapping_fn.mapping_fn());
    let resources = {
        let mut resources = Resources::new();
        resources.insert(1u8);
        Resources::<SetUp>::from(resources)
    };
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("resolve_mapping_fn"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = ParamsSpec::<MockSrc>::mapping_fn(None, test_mapping_fn);

    let mock_src = ValueSpecRt::resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    )?;

    assert_eq!(MockSrc(1), mock_src);
    Ok(())
}

#[test]
fn resolve_mapping_fn_returns_err_when_mutably_borrowed() -> Result<(), ParamsResolveError> {
    let test_mapping_fn = TestMappingFns::MockSrcFromU8AndU16;
    let mut mapping_fn_reg = MappingFnReg::new();
    mapping_fn_reg.insert(test_mapping_fn.id(), test_mapping_fn.mapping_fn());
    let resources = {
        let mut resources = Resources::new();
        resources.insert(1u8);
        resources.insert(2u16);
        Resources::<SetUp>::from(resources)
    };
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("resolve_mapping_fn_returns_err_when_mutably_borrowed"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = ParamsSpec::<MockSrc>::mapping_fn(None, test_mapping_fn);

    let _u8_borrowed = resources.borrow::<u8>();
    let _u16_mut_borrowed = resources.borrow_mut::<u16>();
    let mock_src_result = ValueSpecRt::resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    );

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &mock_src_result,
                    Err(ParamsResolveError::FromMapBorrowConflict { value_resolution_ctx, from_type_name })
                    if value_resolution_ctx.value_resolution_mode() == ValueResolutionMode::Current
                    && value_resolution_ctx.item_id()
                        == &item_id!("resolve_mapping_fn_returns_err_when_mutably_borrowed")
                    && value_resolution_ctx.params_type_name() == "MockSrc"
                    && value_resolution_ctx.resolution_chain().is_empty()
                    && from_type_name == "u16"
                ),
                "expected `mock_src_result` to be \
                `Err(ParamsResolveError::FromMapBorrowConflict {{ .. }})`\n\
                with `resolution_chain`: `[]`,\n\
                but was `{mock_src_result:?}`"
            );
        }
    })();
    Ok(())
}

#[test]
fn resolve_field_wise() -> Result<(), ParamsResolveError> {
    let mapping_fn_reg = MappingFnReg::new();
    let resources = {
        let mut resources = Resources::new();
        resources.insert(1u8);
        Resources::<SetUp>::from(resources)
    };
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("resolve_field_wise"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = MockSrc::field_wise_spec().with_0_in_memory().build();

    let mock_src = ValueSpecRt::resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    )?;

    assert_eq!(MockSrc(1), mock_src);
    Ok(())
}

#[test]
fn resolve_field_wise_returns_err_when_mutably_borrowed() -> Result<(), ParamsResolveError> {
    let mapping_fn_reg = MappingFnReg::new();
    let resources = {
        let mut resources = Resources::new();
        resources.insert(1u8);
        Resources::<SetUp>::from(resources)
    };
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("resolve_field_wise_returns_err_when_mutably_borrowed"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = MockSrc::field_wise_spec().with_0_in_memory().build();

    let _u8_mut_borrowed = resources.borrow_mut::<u8>();
    let mock_src_result = ValueSpecRt::resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    );

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &mock_src_result,
                    Err(ParamsResolveError::InMemoryBorrowConflict { value_resolution_ctx })
                    if value_resolution_ctx.value_resolution_mode() == ValueResolutionMode::Current
                    && value_resolution_ctx.item_id()
                        == &item_id!("resolve_field_wise_returns_err_when_mutably_borrowed")
                    && value_resolution_ctx.params_type_name() == "MockSrc"
                    && value_resolution_ctx.resolution_chain() == [
                        FieldNameAndType::new(String::from("0"), tynm::type_name::<u8>())
                    ]
                ),
                "expected `mock_src_result` to be \
                `Err(ParamsResolveError::InMemoryBorrowConflict {{ .. }})`\n\
                with `resolution_chain`: `[(0, u8)]`,\n\
                but was `{mock_src_result:?}`"
            );
        }
    })();
    Ok(())
}

#[test]
fn try_resolve_stored_param() -> Result<(), ParamsResolveError> {
    let mapping_fn_reg = MappingFnReg::new();
    let resources = {
        let mut resources = Resources::new();
        resources.insert(MockSrc(1));
        Resources::<SetUp>::from(resources)
    };
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("try_resolve_stored_param"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = ParamsSpec::<MockSrc>::Stored;

    let mock_src = ValueSpecRt::try_resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    )?;

    assert_eq!(Some(MockSrc(1)), mock_src);
    Ok(())
}

#[test]
fn try_resolve_in_memory() -> Result<(), ParamsResolveError> {
    let mapping_fn_reg = MappingFnReg::new();
    let resources = {
        let mut resources = Resources::new();
        resources.insert(MockSrc(1));
        Resources::<SetUp>::from(resources)
    };
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("try_resolve_in_memory"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = ParamsSpec::<MockSrc>::InMemory;

    let mock_src = ValueSpecRt::try_resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    )?;

    assert_eq!(Some(MockSrc(1)), mock_src);
    Ok(())
}

#[test]
fn try_resolve_in_memory_returns_none_when_not_found() -> Result<(), ParamsResolveError> {
    let mapping_fn_reg = MappingFnReg::new();
    let resources = Resources::<SetUp>::from(Resources::new());
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("try_resolve_in_memory_returns_none_when_not_found"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = ParamsSpec::<MockSrc>::InMemory;

    let mock_src = ValueSpecRt::try_resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    )?;

    assert_eq!(None, mock_src);
    Ok(())
}

#[test]
fn try_resolve_in_memory_returns_err_when_mutably_borrowed() -> Result<(), ParamsResolveError> {
    let mapping_fn_reg = MappingFnReg::new();
    let resources = {
        let mut resources = Resources::new();
        resources.insert(MockSrc(1));
        Resources::<SetUp>::from(resources)
    };
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("try_resolve_in_memory_returns_err_when_mutably_borrowed"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = ParamsSpec::<MockSrc>::InMemory;

    let _mock_src_mut_borrowed = resources.borrow_mut::<MockSrc>();
    let mock_src_result = ValueSpecRt::try_resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    );

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &mock_src_result,
                    Err(ParamsResolveError::InMemoryBorrowConflict { value_resolution_ctx })
                    if value_resolution_ctx.value_resolution_mode() == ValueResolutionMode::Current
                    && value_resolution_ctx.item_id()
                        == &item_id!("try_resolve_in_memory_returns_err_when_mutably_borrowed")
                    && value_resolution_ctx.params_type_name() == "MockSrc"
                    && value_resolution_ctx.resolution_chain().is_empty()
                ),
                "expected `mock_src_result` to be \
                `Err(ParamsResolveError::InMemoryBorrowConflict {{ .. }})`\n\
                with `resolution_chain`: `[]`,\n\
                but was `{mock_src_result:?}`"
            );
        }
    })();
    Ok(())
}

#[test]
fn try_resolve_value() -> Result<(), ParamsResolveError> {
    let mapping_fn_reg = MappingFnReg::new();
    let resources = Resources::<SetUp>::from(Resources::new());
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("try_resolve_value"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = ParamsSpec::<MockSrc>::Value { value: MockSrc(1) };

    let mock_src = ValueSpecRt::try_resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    )?;

    assert_eq!(Some(MockSrc(1)), mock_src);
    Ok(())
}

#[test]
fn try_resolve_mapping_fn() -> Result<(), ParamsResolveError> {
    let test_mapping_fn = TestMappingFns::MockSrcFromU8;
    let mut mapping_fn_reg = MappingFnReg::new();
    mapping_fn_reg.insert(test_mapping_fn.id(), test_mapping_fn.mapping_fn());
    let resources = {
        let mut resources = Resources::new();
        resources.insert(1u8);
        Resources::<SetUp>::from(resources)
    };
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("try_resolve_mapping_fn"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = ParamsSpec::<MockSrc>::mapping_fn(None, test_mapping_fn);

    let mock_src = ValueSpecRt::try_resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    )?;

    assert_eq!(Some(MockSrc(1)), mock_src);
    Ok(())
}

#[test]
fn try_resolve_mapping_fn_returns_err_when_mutably_borrowed() -> Result<(), ParamsResolveError> {
    let test_mapping_fn = TestMappingFns::MockSrcFromU8AndU16;
    let mut mapping_fn_reg = MappingFnReg::new();
    mapping_fn_reg.insert(test_mapping_fn.id(), test_mapping_fn.mapping_fn());
    let resources = {
        let mut resources = Resources::new();
        resources.insert(1u8);
        resources.insert(2u16);
        Resources::<SetUp>::from(resources)
    };
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("try_resolve_mapping_fn_returns_err_when_mutably_borrowed"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec =
        ParamsSpec::<MockSrc>::mapping_fn(None, TestMappingFns::MockSrcFromU8AndU16);

    let _u8_borrowed = resources.borrow::<u8>();
    let _u16_mut_borrowed = resources.borrow_mut::<u16>();
    let mock_src_result = ValueSpecRt::try_resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    );

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &mock_src_result,
                    Err(ParamsResolveError::FromMapBorrowConflict { value_resolution_ctx, from_type_name })
                    if value_resolution_ctx.value_resolution_mode() == ValueResolutionMode::Current
                    && value_resolution_ctx.item_id()
                        == &item_id!("try_resolve_mapping_fn_returns_err_when_mutably_borrowed")
                    && value_resolution_ctx.params_type_name() == "MockSrc"
                    && value_resolution_ctx.resolution_chain().is_empty()
                    && from_type_name == "u16"
                ),
                "expected `mock_src_result` to be \
                `Err(ParamsResolveError::FromMapBorrowConflict {{ .. }})`\n\
                with `resolution_chain`: `[]`,\n\
                but was `{mock_src_result:?}`"
            );
        }
    })();
    Ok(())
}

#[test]
fn try_resolve_field_wise() -> Result<(), ParamsResolveError> {
    let mapping_fn_reg = MappingFnReg::new();
    let resources = {
        let mut resources = Resources::new();
        resources.insert(1u8);
        Resources::<SetUp>::from(resources)
    };
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("try_resolve_field_wise"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = MockSrc::field_wise_spec().with_0_in_memory().build();

    let mock_src = ValueSpecRt::try_resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    )?;

    assert_eq!(Some(MockSrc(1)), mock_src);
    Ok(())
}

#[test]
fn try_resolve_field_wise_returns_err_when_mutably_borrowed() -> Result<(), ParamsResolveError> {
    let mapping_fn_reg = MappingFnReg::new();
    let resources = {
        let mut resources = Resources::new();
        resources.insert(1u8);
        Resources::<SetUp>::from(resources)
    };
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::Current,
        item_id!("try_resolve_field_wise_returns_err_when_mutably_borrowed"),
        tynm::type_name::<MockSrc>(),
    );
    let mock_src_spec = MockSrc::field_wise_spec().with_0_in_memory().build();

    let _u8_mut_borrowed = resources.borrow_mut::<u8>();
    let mock_src_result = &ValueSpecRt::try_resolve(
        &mock_src_spec,
        &mapping_fn_reg,
        &resources,
        &mut value_resolution_ctx,
    );

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                matches!(
                    &mock_src_result,
                    Err(ParamsResolveError::InMemoryBorrowConflict { value_resolution_ctx })
                    if value_resolution_ctx.value_resolution_mode() == ValueResolutionMode::Current
                    && value_resolution_ctx.item_id()
                        == &item_id!("try_resolve_field_wise_returns_err_when_mutably_borrowed")
                    && value_resolution_ctx.params_type_name() == "MockSrc"
                    && value_resolution_ctx.resolution_chain() == [
                        FieldNameAndType::new(String::from("0"), tynm::type_name::<u8>())
                    ]
                ),
                "expected `mock_src_result` to be \
                `Err(ParamsResolveError::InMemoryBorrowConflict {{ .. }})`\n\
                with `resolution_chain`: `[(0, u8)]`,\n\
                but was `{mock_src_result:?}`"
            );
        }
    })();
    Ok(())
}

#[test]
fn merge_stored_with_other_uses_other() {
    let mut params_spec_a = ParamsSpec::<MockSrc>::Stored;
    let params_spec_b = AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::InMemory);

    params_spec_a.merge(&*params_spec_b);

    assert!(matches!(&params_spec_a, ParamsSpec::<MockSrc>::InMemory));
}

#[test]
fn merge_value_with_other_no_change() {
    let mut params_spec_a = ParamsSpec::<MockSrc>::Value { value: MockSrc(1) };
    let params_spec_b = AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::mapping_fn(
        None,
        TestMappingFns::MockSrcFromU8,
    ));

    params_spec_a.merge(&*params_spec_b);

    assert!(
        matches!(&params_spec_a, ParamsSpec::<MockSrc>::Value { value } if value == &MockSrc(1))
    );
}

#[test]
fn merge_in_memory_with_other_no_change() {
    let mut params_spec_a = ParamsSpec::<MockSrc>::InMemory;
    let params_spec_b = AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::mapping_fn(
        None,
        TestMappingFns::MockSrcFromU8,
    ));

    params_spec_a.merge(&*params_spec_b);

    assert!(matches!(&params_spec_a, ParamsSpec::<MockSrc>::InMemory));
}

#[test]
fn merge_mapping_fn_with_other_no_change() {
    let mut params_spec_a = ParamsSpec::<MockSrc>::mapping_fn(None, TestMappingFns::MockSrcFromU8);
    let params_spec_b = AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::InMemory);

    params_spec_a.merge(&*params_spec_b);

    assert!(matches!(
        &params_spec_a,
        ParamsSpec::<MockSrc>::MappingFn { field_name: None, mapping_fn_id }
        if mapping_fn_id == &TestMappingFns::MockSrcFromU8.id(),
    ));
}

#[test]
fn merge_field_wise_with_stored_no_change() {
    let mut params_spec_a = MockSrc::field_wise_spec().with_0_in_memory().build();
    let params_spec_b = AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::Stored);

    params_spec_a.merge(&*params_spec_b);

    assert!(matches!(
        &params_spec_a,
        ParamsSpec::<MockSrc>::FieldWise { field_wise_spec: MockSrcFieldWise(f0) }
        if matches!(f0, ValueSpec::InMemory)
    ));
}

#[test]
fn merge_field_wise_with_value_no_change() {
    let mut params_spec_a = MockSrc::field_wise_spec().with_0_in_memory().build();
    let params_spec_b = AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::Value { value: MockSrc(1) });

    params_spec_a.merge(&*params_spec_b);

    assert!(matches!(
        &params_spec_a,
        ParamsSpec::<MockSrc>::FieldWise { field_wise_spec: MockSrcFieldWise(f0) }
        if matches!(f0, ValueSpec::InMemory)
    ));
}

#[test]
fn merge_field_wise_with_in_memory_no_change() {
    let mut params_spec_a = MockSrc::field_wise_spec().with_0_in_memory().build();
    let params_spec_b = AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::InMemory);

    params_spec_a.merge(&*params_spec_b);

    assert!(matches!(
        &params_spec_a,
        ParamsSpec::<MockSrc>::FieldWise { field_wise_spec: MockSrcFieldWise(f0) }
        if matches!(f0, ValueSpec::InMemory)
    ));
}

#[test]
fn merge_field_wise_with_from_mapping_fn_no_change() {
    let mut params_spec_a = MockSrc::field_wise_spec().with_0_in_memory().build();
    let params_spec_b = AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::mapping_fn(
        None,
        TestMappingFns::MockSrcFromU8,
    ));

    params_spec_a.merge(&*params_spec_b);

    assert!(matches!(
        &params_spec_a,
        ParamsSpec::<MockSrc>::FieldWise { field_wise_spec: MockSrcFieldWise(f0) }
        if matches!(f0, ValueSpec::InMemory)
    ));
}

#[test]
fn merge_field_wise_with_field_wise_deep_merges() {
    let mut params_spec_a = MockSrc::field_wise_spec().build();
    let params_spec_b = AnySpecRtBoxed::new(MockSrc::field_wise_spec().with_0_in_memory().build());

    params_spec_a.merge(&*params_spec_b);

    assert!(matches!(
        &params_spec_a,
        ParamsSpec::<MockSrc>::FieldWise { field_wise_spec: MockSrcFieldWise(f0) }
        if matches!(f0, ValueSpec::InMemory)
    ));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Sequence)]
#[enum_iterator(crate = peace::enum_iterator)]
enum TestMappingFns {
    MockSrcFromU8,
    MockSrcFromU8AndU16,
    VecU8FromBoolAndU16,
}

impl MappingFns for TestMappingFns {
    fn id(self) -> MappingFnId {
        let name = match self {
            TestMappingFns::MockSrcFromU8 => "MockSrcFromU8",
            TestMappingFns::MockSrcFromU8AndU16 => "MockSrcFromU8AndU16",
            TestMappingFns::VecU8FromBoolAndU16 => "VecU8FromBoolAndU16",
        };
        MappingFnId::new(name.into())
    }

    fn mapping_fn(self) -> Box<dyn MappingFn> {
        match self {
            TestMappingFns::MockSrcFromU8 => MappingFnImpl::from_func(|n: &u8| Some(MockSrc(*n))),
            TestMappingFns::MockSrcFromU8AndU16 => {
                MappingFnImpl::from_func(|n: &u8, _: &u16| Some(MockSrc(*n)))
            }
            TestMappingFns::VecU8FromBoolAndU16 => {
                MappingFnImpl::from_func(|_: &bool, _: &u16| Some(VecA::default()))
            }
        }
    }
}
