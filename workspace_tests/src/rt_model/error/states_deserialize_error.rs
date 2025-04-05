use miette::{Diagnostic, NamedSource};
use peace::{flow_model::flow_id, rt_model::StatesDeserializeError};
use serde::de::Error;

#[test]
fn coverage_diagnostic() {
    let error = Box::new(StatesDeserializeError {
        flow_id: flow_id!("test_flow"),
        states_file_source: NamedSource::new("states_current.yaml", String::from("vec_item: ~")),
        error_span: None,
        error_message: String::from("Failed to deserialize states."),
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
