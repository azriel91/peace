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
//! :                            : | * MyParamsSpec             | :    .               :
//! :                            : | * MyParamsSpecPartial      | :    |               :
//! :                            : | * MyParamsSpecBuilder      | : <--'               :
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
pub use peace_params_derive::Params;

pub use crate::{params::Params, params_spec_builder::ParamsSpecBuilder, value_spec::ValueSpec};

mod params;
mod params_spec_builder;
mod value_spec;
