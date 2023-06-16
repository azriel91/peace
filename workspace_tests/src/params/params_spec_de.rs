use peace::params::{Params, ParamsSpec, ParamsSpecDe};

use crate::mock_item::MockSrc;

#[test]
fn debug() -> Result<(), serde_yaml::Error> {
    assert_eq!("Stored", format!("{:?}", ParamsSpecDe::<MockSrc>::Stored));
    assert_eq!(
        "Value(MockSrc(1))",
        format!("{:?}", ParamsSpecDe::<MockSrc>::Value { value: MockSrc(1) })
    );
    assert_eq!(
        "InMemory",
        format!("{:?}", ParamsSpecDe::<MockSrc>::InMemory)
    );
    assert_eq!(
        "MappingFn(MappingFnImpl { \
            field_name: Some(\"field\"), \
            fn_map: \"None\", \
            marker: PhantomData<(workspace_tests::mock_item::MockSrc, ((),))> \
        })",
        format!(
            "{:?}",
            ParamsSpecDe::<MockSrc>::MappingFn(serde_yaml::from_str(
                "field_name: !Some field\n\
                fn_map: Fn(&u8) -> MockSrc\n\
                marker: ~\n"
            )?)
        )
    );
    let ParamsSpec::FieldWise { field_wise_spec } = <MockSrc as Params>::field_wise_spec().build()
    else {
        panic!("field_wise_spec().build() should return `ParamsSpec::FieldWise`");
    };
    assert_eq!(
        "FieldWise(MockSrcFieldWise(Stored))",
        format!(
            "{:?}",
            ParamsSpecDe::<MockSrc>::FieldWise { field_wise_spec }
        )
    );

    Ok(())
}
