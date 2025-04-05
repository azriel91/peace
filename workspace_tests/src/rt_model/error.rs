use std::fmt;

use peace::{
    item_model::item_id,
    miette::{Diagnostic, GraphicalReportHandler, GraphicalTheme},
    params::{ParamsSpec, ParamsSpecs},
    rt_model::Error,
};

use crate::mock_item::MockSrc;

mod params_specs_deserialize_error;
mod states_deserialize_error;

#[test]
fn params_specs_mismatch_display_with_all_error_cases() -> fmt::Result {
    let item_ids_with_no_params_specs = vec![item_id!("no_params_0"), item_id!("no_params_1")];
    let params_specs_provided_mismatches = {
        let mut params_specs_provided_mismatches = ParamsSpecs::new();
        params_specs_provided_mismatches.insert(
            item_id!("params_spec_provided_with_no_item_0"),
            ParamsSpec::<MockSrc>::InMemory,
        );
        params_specs_provided_mismatches.insert(
            item_id!("params_spec_provided_with_no_item_1"),
            ParamsSpec::<MockSrc>::Stored,
        );
        Box::new(params_specs_provided_mismatches)
    };
    let params_specs_stored_mismatches = {
        let mut params_specs_stored_mismatches = ParamsSpecs::new();
        params_specs_stored_mismatches.insert(
            item_id!("params_spec_stored_with_no_item_0"),
            ParamsSpec::<MockSrc>::InMemory,
        );
        params_specs_stored_mismatches.insert(
            item_id!("params_spec_stored_with_no_item_1"),
            ParamsSpec::<MockSrc>::Stored,
        );
        Box::new(Some(params_specs_stored_mismatches))
    };
    let params_specs_not_usable = vec![
        item_id!("stored_mapping_fn_0"),
        item_id!("stored_mapping_fn_1"),
    ];

    let error = Error::ParamsSpecsMismatch {
        item_ids_with_no_params_specs,
        params_specs_provided_mismatches,
        params_specs_stored_mismatches,
        params_specs_not_usable,
    };

    let report_handler = GraphicalReportHandler::new()
        .without_cause_chain()
        .with_theme(GraphicalTheme::none());
    let mut err_buffer = String::with_capacity(1280);
    report_handler.render_report(&mut err_buffer, &error)?;
    let err_buffer = err_buffer.lines().fold(
        String::with_capacity(err_buffer.len()),
        |mut buffer, line| {
            if line.trim().is_empty() {
                buffer.push('\n');
            } else {
                buffer.push_str(line);
                buffer.push('\n');
            }
            buffer
        },
    );

    assert_eq!(
        r#"peace_rt_model::params_specs_mismatch

  x Item params specs do not match with the items in the flow.
  help: The following items do not have parameters provided:

        * no_params_0
        * no_params_1

        The following provided params specs do not correspond to any items in the flow:

        * params_spec_provided_with_no_item_0
        * params_spec_provided_with_no_item_1

        The following stored params specs do not correspond to any items in the flow:

        * params_spec_stored_with_no_item_0
        * params_spec_stored_with_no_item_1

        The following items either have not had a params spec provided previously,
        or had contained a mapping function, which cannot be loaded from disk.

        So the params spec needs to be provided to the command context for:

        * stored_mapping_fn_0
        * stored_mapping_fn_1

"#,
        err_buffer
    );

    Ok(())
}

#[test]
fn coverage_diagnostic() {
    let error = Box::new(Error::StatesCurrentDiscoverRequired);

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
