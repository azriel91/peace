use miette::{Diagnostic, NamedSource};
use peace::{cfg::profile, flow_model::flow_id, rt_model::ParamsSpecsDeserializeError};
use serde::de::Error;

#[test]
fn coverage_diagnostic() {
    let error = Box::new(ParamsSpecsDeserializeError {
        profile: profile!("test_profile"),
        flow_id: flow_id!("test_flow"),
        params_specs_file_source: NamedSource::new(
            "params_specs.yaml",
            String::from("vec_item: ~"),
        ),
        error_span: None,
        error_message: String::from("params specs was not provided for `VecCopyItem`."),
        context_span: None,
        error: serde_yaml::Error::custom("custom error"),
    });

    std::borrow::Borrow::<dyn miette::Diagnostic + '_>::borrow(&error);

    let _ = error.code();
    let _ = error.severity();
    let _ = error.help();
    let _ = error.url();
    let _ = error.source_code();
    let _ = error.labels();
    let _ = error.related();
    let _ = error.diagnostic_source();
}
