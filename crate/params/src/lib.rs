//! Constraints and specifications for parameters for the peace automation
//! framework.
//!
//! This crate defines types and traits for implementors and users to work with
//! item spec params.
//!
//! # Design
//!
//! When an item spec is defined, implementors define the parameters type for
//! that item spec.
//!
//! For Peace to derive additional functionality from that type, this crate:
//!
//! * Defines the `Params` trait to bridge between the parameters type and
//!   associated types.
//! * Re-exports the `Params` derive macro which implements the `Params` trait.
//!
//! ## How It Fits Together
//!
//! ```text
//! .----------------------------------------------------------------------------------.
//! :          Users             :         Implementors           :       Peace        :
//! :----------------------------:--------------------------------:--------------------:
//! :                            :                                :                    :
//! :                            :   .-------------------.        :                    :
//! :                            :   | #[derive(Params)] |        : ---.               :
//! :                            :   | struct MyParams;  |        :    |               :
//! :                            :   '-------------------'        :    '               :
//! :                            :                                :  proc macro        :
//! :                            : .----------------------------. :  generates         :
//! :                            : | * MyParamsFieldWise        | :    .               :
//! :                            : | * MyParamsPartial          | :    |               :
//! :                            : | * MyParamsFieldWiseBuilder | : <--'               :
//! :                            : | * impl Params for MyParams | :                    :
//! :                            : '----------------------------' :                    :
//! :                            :                                :                    :
//! :                            :   .--------------------.       :                    :
//! :                            :   | struct MyItemSpec; |       :                    :
//! :                            :   |                    |       : ---.               :
//! :                            :   | impl ItemSpec for  |       :    |               :
//! :                            :   |   MyItemSpec {     |       :    |               :
//! :                            :   |     type Params =  |       :    '               :
//! :                            :   |     MyParams;      |       :  exposes API       :
//! :                            :   | }                  |       :  with constraints  :
//! :                            :   '--------------------'       :  from              :
//! :                            :                                :  <ItemSpec::Params :
//! : .------------------------. :                                :    as Params>      :
//! : | cmd_ctx_builder        | :                                :    .               :
//! : | .with_item_spec_params | <-------------------------------------'               :
//! : |    ::<IS>(             | :                                :                    :
//! : |     item_spec_id,      | :                                :                    :
//! : |     my_p_spec_builder  | :                                :                    :
//! : |       .with_f(123)     | :                                :                    :
//! : |       .with_from(..)   | :                                :                    :
//! : |     /* .build() */     | :                                :                    :
//! : |   )                    | :                                :                    :
//! : '------------------------' :                                :                    :
//! :                            :                                :                    :
//! '----------------------------------------------------------------------------------'
//! ```

// Re-exports
pub use peace_params_derive::{value_impl, ValueSpec, ValueSpecFieldless};
pub use tynm;

pub use crate::{
    field_name_and_type::FieldNameAndType, field_wise_spec_rt::FieldWiseSpecRt,
    mapping_fn::MappingFn, mapping_fn_impl::MappingFnImpl,
    params_resolve_error::ParamsResolveError, params_specs::ParamsSpecs, value::Value,
    value_fieldless::ValueFieldless, value_resolution_ctx::ValueResolutionCtx,
    value_resolution_mode::ValueResolutionMode, value_spec::ValueSpec, value_spec_de::ValueSpecDe,
    value_spec_fieldless::ValueSpecFieldless, value_spec_fieldless_de::ValueSpecFieldlessDe,
    value_spec_rt::ValueSpecRt,
};

mod field_name_and_type;
mod field_wise_spec_rt;
mod mapping_fn;
mod mapping_fn_impl;
mod params_resolve_error;
mod params_specs;
mod std_impl;
mod value;
mod value_fieldless;
mod value_resolution_ctx;
mod value_resolution_mode;
mod value_spec;
mod value_spec_de;
mod value_spec_fieldless;
mod value_spec_fieldless_de;
mod value_spec_rt;
