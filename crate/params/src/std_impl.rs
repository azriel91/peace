//! Trait and struct impls for standard library types.
#![allow(non_camel_case_types)]

#[cfg(not(target_arch = "wasm32"))]
use std::ffi::OsString;
use std::path::PathBuf;

use peace_params_derive::value_impl;

// IMPORTANT!
//
// When updating the types that implement `ParamsFieldless`, make sure to update
// `params_derive/src/util.rs#STD_LIB_TYPES`.
//
// These are the types that we don't require users to annotate with
// `#[value_spec(fieldless)]`, but will be treated as such.

impl_value_for!(
    bool, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize, String, PathBuf,
);

// WASM doesn't support serialization of `OsString`s.
#[cfg(not(target_arch = "wasm32"))]
value_impl!(
    #[crate_internal]
    #[value_spec(fieldless)]
    struct OsString;
);

value_impl!(
    #[crate_internal]
    #[value_spec(fieldless)]
    struct Option<T>
    where
        T: Clone + std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned;
);

value_impl!(
    #[crate_internal]
    #[value_spec(fieldless)]
    struct Vec<T>
    where
        T: Clone + std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned;
);

macro_rules! impl_value_for {
    ($($T:ident),*,) => {
        $(
            value_impl!(
                #[crate_internal]
                #[value_spec(fieldless)]
                struct $T;
            );
        )*
    }
}

use impl_value_for;
