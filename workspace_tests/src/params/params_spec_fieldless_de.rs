use peace::params::ParamsSpecFieldlessDe;

use crate::mock_item::MockSrc;

#[test]
fn clone() {
    let _params_spec_fieldless_de =
        ParamsSpecFieldlessDe::<MockSrc>::Value { value: MockSrc(1) }.clone();
}

#[test]
fn debug() -> Result<(), serde_yaml::Error> {
    assert_eq!(
        "Stored",
        format!("{:?}", ParamsSpecFieldlessDe::<MockSrc>::Stored)
    );
    assert_eq!(
        "Value(MockSrc(1))",
        format!(
            "{:?}",
            ParamsSpecFieldlessDe::<MockSrc>::Value { value: MockSrc(1) }
        )
    );
    assert_eq!(
        "InMemory",
        format!("{:?}", ParamsSpecFieldlessDe::<MockSrc>::InMemory)
    );
    assert_eq!(
        "MappingFn(MappingFnImpl { \
            field_name: Some(\"field\"), \
            fn_map: \"None\", \
            marker: PhantomData<(workspace_tests::mock_item::MockSrc, ((),))> \
        })",
        format!(
            "{:?}",
            ParamsSpecFieldlessDe::<MockSrc>::MappingFn(serde_yaml::from_str(
                "field_name: !Some field\n\
                fn_map: Fn(&u8) -> MockSrc\n\
                marker: ~\n"
            )?)
        )
    );

    Ok(())
}
