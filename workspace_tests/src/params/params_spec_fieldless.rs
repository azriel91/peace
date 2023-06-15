use peace::params::{AnySpecRt, ParamsFieldless, ParamsSpecFieldless};

#[test]
fn serialize_stored() -> Result<(), serde_yaml::Error> {
    let u8_spec: <u8 as ParamsFieldless>::Spec = <u8 as ParamsFieldless>::Spec::Stored;
    assert_eq!(
        r#"Stored
"#,
        serde_yaml::to_string(&u8_spec)?,
    );

    Ok(())
}

#[test]
fn serialize_value() -> Result<(), serde_yaml::Error> {
    let u8_spec: <u8 as ParamsFieldless>::Spec = 1u8.into();
    assert_eq!(
        r#"!Value
value: 1
"#,
        serde_yaml::to_string(&u8_spec)?,
    );

    Ok(())
}

#[test]
fn serialize_in_memory() -> Result<(), serde_yaml::Error> {
    let u8_spec: <u8 as ParamsFieldless>::Spec = ParamsSpecFieldless::<u8>::InMemory;
    assert_eq!(
        r#"InMemory
"#,
        serde_yaml::to_string(&u8_spec)?,
    );

    Ok(())
}

#[test]
fn serialize_from_map() -> Result<(), serde_yaml::Error> {
    let u8_spec: <u8 as ParamsFieldless>::Spec =
        ParamsSpecFieldless::<u8>::from_map(None, |_: &bool, _: &u16| Some(1u8));
    assert_eq!(
        r#"!MappingFn
field_name: null
fn_map: Some(Fn(&bool, &u16) -> Option<u8>)
marker: null
"#,
        serde_yaml::to_string(&u8_spec)?,
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
        ParamsSpecFieldless::<u8>::Stored
    ));

    Ok(())
}

#[test]
fn deserialize_value() -> Result<(), serde_yaml::Error> {
    assert!(matches!(
        serde_yaml::from_str(
            r#"!Value
value: 1
"#
        )?,
        ParamsSpecFieldless::<u8>::Value { value }
        if value == 1u8
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
        #[cfg_attr(coverage_nightly, no_coverage)]
        || {
            assert!(
                matches!(&deserialized, ParamsSpecFieldless::<u8>::InMemory),
                "was {deserialized:?}"
            );
        }
    })();

    Ok(())
}

#[test]
fn deserialize_from_map() -> Result<(), serde_yaml::Error> {
    let deserialized = serde_yaml::from_str(
        r#"!MappingFn
field_name: null
fn_map: Some(Fn(&bool, &u16) -> Option<Vec<u8>>)
marker: null
"#,
    )?;

    ({
        #[cfg_attr(coverage_nightly, no_coverage)]
        || {
            assert!(
                matches!(
                    &deserialized,
                    ParamsSpecFieldless::<u8>::MappingFn(mapping_fn)
                    if !mapping_fn.is_valued()
                ),
                "was {deserialized:?}"
            );
        }
    })();

    Ok(())
}

#[test]
fn is_usable_returns_false_for_stored() {
    assert!(!ParamsSpecFieldless::<u8>::Stored.is_usable());
}

#[test]
fn is_usable_returns_true_for_value_and_in_memory() {
    assert!(ParamsSpecFieldless::<u8>::Value { value: 1u8 }.is_usable());
    assert!(ParamsSpecFieldless::<u8>::InMemory.is_usable());
}

#[test]
fn is_usable_returns_true_when_mapping_fn_is_some() {
    assert!(ParamsSpecFieldless::<u8>::from_map(None, |_: &u8| None).is_usable());
}

#[test]
fn is_usable_returns_false_when_mapping_fn_is_none() -> Result<(), serde_yaml::Error> {
    let params_spec: ParamsSpecFieldless<u8> = serde_yaml::from_str(
        r#"!MappingFn
field_name: null
fn_map: Some(Fn(&bool, &u16) -> Option<u8>)
marker: null
"#,
    )?;

    assert!(!params_spec.is_usable());
    Ok(())
}
