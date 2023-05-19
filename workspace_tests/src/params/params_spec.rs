use peace::params::Params;

use crate::vec_copy_item_spec::VecA;

#[test]
fn serialize() -> Result<(), serde_yaml::Error> {
    let mut vec_a_spec: <VecA as Params>::Spec = VecA(vec![1u8]).into();
    assert_eq!(
        r#"!Value
- 1
"#,
        serde_yaml::to_string(&vec_a_spec)?,
    );

    vec_a_spec = VecA::field_wise_spec().with_0(vec![1u8]).build();
    assert_eq!(
        r#"!FieldWise
Value:
- 1
"#,
        serde_yaml::to_string(&vec_a_spec)?,
    );

    Ok(())
}
