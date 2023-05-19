use peace::{
    cfg::{item_spec_id, ItemSpecId},
    params::{
        FieldWiseSpecRt, Params, ParamsSpec, ValueResolutionCtx, ValueResolutionMode, ValueSpec,
    },
    resources::{resources::ts::SetUp, Resources},
};

use crate::vec_copy_item_spec::{VecA, VecAFieldWise};

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
fn serialize_from_map() -> Result<(), serde_yaml::Error> {
    let vec_a_spec: <VecA as Params>::Spec =
        <VecA as Params>::Spec::from_map(None, |_: &bool, _: &u16| Some(VecA(vec![1u8])));
    assert_eq!(
        r#"!MappingFn
field_name: null
fn_map: Some(Fn(&bool, &u16) -> Option<VecA>)
marker: null
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
fn serialize_field_wise_from_map() -> Result<(), serde_yaml::Error> {
    let vec_a_spec: <VecA as Params>::Spec = VecA::field_wise_spec()
        .with_0_from_map(|_: &bool, _: &u16| Some(vec![1u8]))
        .build();
    assert_eq!(
        r#"!FieldWise
field_wise_spec: !MappingFn
  field_name: _0
  fn_map: Some(Fn(&bool, &u16) -> Option<Vec<u8>>)
  marker: null
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

    assert!(
        matches!(&deserialized, ParamsSpec::<VecA>::InMemory),
        "was {deserialized:?}"
    );

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

    assert!(
        matches!(
            &deserialized,
            ParamsSpec::<VecA>::MappingFn(mapping_fn)
            if !mapping_fn.is_valued()
        ),
        "was {deserialized:?}"
    );

    Ok(())
}

#[test]
fn deserialize_field_wise_value() -> Result<(), Box<dyn std::error::Error>> {
    let resources = Resources::<SetUp>::from(Resources::new());
    let mut value_resolution_ctx = ValueResolutionCtx::new(
        ValueResolutionMode::ApplyDry,
        item_spec_id!("deserialize_field_wise"),
        tynm::type_name::<VecA>(),
    );

    let deserialized = serde_yaml::from_str(
        r#"!FieldWise
field_wise_spec: !Value
  value:
  - 1
"#,
    )?;

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
            && field_wise_spec.resolve(&resources, &mut value_resolution_ctx)?
            == VecA(vec![1u8])
        ),
        "was {deserialized:?}"
    );

    Ok(())
}

#[test]
fn deserialize_field_wise_in_memory() -> Result<(), serde_yaml::Error> {
    let deserialized = serde_yaml::from_str(
        r#"!FieldWise
field_wise_spec: InMemory
"#,
    )?;

    assert!(
        matches!(
            &deserialized,
            ParamsSpec::<VecA>::FieldWise {
                field_wise_spec: VecAFieldWise(ValueSpec::<Vec<u8>>::InMemory)
            }
        ),
        "was {deserialized:?}"
    );

    Ok(())
}

#[test]
fn deserialize_field_wise_from_map() -> Result<(), serde_yaml::Error> {
    let deserialized = serde_yaml::from_str(
        r#"!FieldWise
field_wise_spec: !MappingFn
  field_name: _0
  fn_map: Some(Fn(&bool, &u16) -> Option<Vec<u8>>)
  marker: null
"#,
    )?;

    assert!(
        matches!(
            &deserialized,
            ParamsSpec::<VecA>::FieldWise {
                field_wise_spec: VecAFieldWise(ValueSpec::<Vec<u8>>::MappingFn(mapping_fn))
            }
            if !mapping_fn.is_valued()
        ),
        "was {deserialized:?}"
    );

    Ok(())
}
