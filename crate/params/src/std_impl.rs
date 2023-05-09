//! Trait and struct impls for standard library types.
#![allow(non_camel_case_types)]

use std::path::PathBuf;

use peace_params_derive::params_impl;

impl_params_for!(
    bool, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize, String, PathBuf,
);

params_impl!(
    #[crate_internal]
    #[params(external)]
    struct Option<T>
    where
        T: Clone + std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned;
);

params_impl!(
    #[crate_internal]
    #[params(external)]
    struct Vec<T>
    where
        T: Clone + std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned;
);

macro_rules! impl_params_for {
    ($($T:ident),*,) => {
        $(
            params_impl!(
                #[crate_internal]
                #[params(external)]
                struct $T;
            );
        )*
    }
}

use impl_params_for;
