use peace::params::ValueSpecDe;

use crate::mock_item::MockSrc;

#[test]
fn debug() -> Result<(), serde_yaml::Error> {
    assert_eq!("Stored", format!("{:?}", ValueSpecDe::<MockSrc>::Stored));
    assert_eq!(
        "Value(MockSrc(1))",
        format!("{:?}", ValueSpecDe::<MockSrc>::Value(MockSrc(1)))
    );
    assert_eq!(
        "InMemory",
        format!("{:?}", ValueSpecDe::<MockSrc>::InMemory)
    );
    assert_eq!(
        "MappingFn(MappingFnImpl { \
            field_name: Some(\"field\"), \
            fn_map: \"None\", \
            marker: PhantomData<(workspace_tests::mock_item::MockSrc, ((),))> \
        })",
        format!(
            "{:?}",
            ValueSpecDe::<MockSrc>::MappingFn(serde_yaml::from_str(
                "field_name: !Some field\n\
                fn_map: Fn(&u8) -> MockSrc\n\
                marker: ~\n"
            )?)
        )
    );

    Ok(())
}
