use std::any::TypeId;

use peace::{
    params::{AnySpecDataType, AnySpecRtBoxed, ParamsSpec},
    resources::type_reg::untagged::{BoxDataTypeDowncast, DataType, DataTypeWrapper},
};

use crate::mock_item::MockSrc;

#[test]
fn clone() {
    let _any_spec_rt_boxed = Clone::clone(&AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::Stored));
}

#[test]
fn debug() {
    let any_spec_rt_boxed = AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::Stored);

    assert_eq!("AnySpecRtBoxed(Stored)", format!("{any_spec_rt_boxed:?}"));
}

#[test]
fn into_inner() {
    let boxed_any_spec_rt = AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::Stored).into_inner();
    let params_spec: &ParamsSpec<MockSrc> = boxed_any_spec_rt.downcast_ref().unwrap_or_else(
        #[cfg_attr(coverage_nightly, coverage(off))]
        || panic!("Expected to downcast `boxed_any_spec_rt` to `ParamsSpec<MockSrc>`."),
    );

    assert!(matches!(params_spec, ParamsSpec::<MockSrc>::Stored));
}

#[test]
fn downcast_mut() {
    let mut any_spec_rt_boxed = AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::Stored);
    let params_spec: &mut ParamsSpec<MockSrc> =
        BoxDataTypeDowncast::downcast_mut(&mut any_spec_rt_boxed).unwrap_or_else(
            #[cfg_attr(coverage_nightly, coverage(off))]
            || panic!("Expected to downcast `any_spec_rt_boxed` to `ParamsSpec<MockSrc>`."),
        );

    assert!(matches!(params_spec, ParamsSpec::<MockSrc>::Stored));
}

#[test]
fn data_type_wrapper_type_name() {
    let any_spec_rt_boxed = AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::Stored);

    assert_eq!(
        "peace_params::params_spec::ParamsSpec<workspace_tests::mock_item::MockSrc>",
        format!("{}", DataTypeWrapper::type_name(&any_spec_rt_boxed)),
    );
}

#[test]
fn data_type_wrapper_clone() {
    let _any_spec_rt_boxed =
        DataTypeWrapper::clone(&AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::Stored));
}

#[test]
fn data_type_wrapper_debug() {
    let any_spec_rt_boxed = AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::Stored);

    assert_eq!(
        "Stored",
        format!("{:?}", DataTypeWrapper::debug(&any_spec_rt_boxed))
    );
}

#[test]
fn data_type_wrapper_inner() {
    let any_spec_rt_boxed = AnySpecRtBoxed::new(ParamsSpec::<MockSrc>::Stored);

    let data_type = DataTypeWrapper::inner(&any_spec_rt_boxed);
    let data_type_type_id = data_type.type_id_inner();

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            // Not sure if type ID of `Box<dyn AnySpecDataType>` is useful.
            assert_eq!(
                TypeId::of::<Box<dyn AnySpecDataType>>(),
                data_type_type_id,
                "type name: {}",
                DataType::type_name(data_type)
            );
        }
    })();
}
