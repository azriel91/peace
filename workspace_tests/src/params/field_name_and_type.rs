use peace::params::FieldNameAndType;

#[test]
fn debug() {
    let field_name_and_type = FieldNameAndType::new(String::from("field"), String::from("Type"));

    assert_eq!(
        "FieldNameAndType { field_name: \"field\", type_name: \"Type\" }",
        format!("{field_name_and_type:?}")
    );
}
