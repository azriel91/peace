use peace::params::{AnySpecRt, AnySpecRtBoxed, ParamsSpec};

use crate::mock_item::MockSrc;

#[test]
fn is_usable_returns_false_for_non_valued_from_mapping_fn() -> Result<(), serde_yaml::Error> {
    let params_spec: ParamsSpec<MockSrc> = serde_yaml::from_str(
        r#"!MappingFn
field_name: null
fn_map: Some(Fn(&bool, &u16) -> Option<u8>)
marker: null
"#,
    )?;

    assert!(!AnySpecRt::is_usable(&Box::new(params_spec)));
    Ok(())
}

#[test]
fn is_usable_returns_true_for_usable_params_spec() {
    let params_spec: ParamsSpec<MockSrc> = ParamsSpec::<MockSrc>::InMemory;

    assert!(AnySpecRt::is_usable(&params_spec));
}

#[test]
fn merge_delegates_to_underlying_type() {
    let mut params_spec_a = Box::new(ParamsSpec::<MockSrc>::Stored);
    let params_spec_b = AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::InMemory);

    AnySpecRt::merge(&mut params_spec_a, &*params_spec_b);

    assert!(matches!(&*params_spec_a, ParamsSpec::<MockSrc>::InMemory));
}
