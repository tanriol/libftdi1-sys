// SPDX-License-Identifier: MIT

//! libftdi1 is a library for working with FTDI chips like FT232BM, FT245BM,
//! FT2232C, FT2232D, FT245R, FT232H, FT230X The documentation for it is
//! available at http://www.intra2net.com/en/developer/libftdi/documentation/
//!
//! This is the raw bindgen-generated binding crate. For more detail see README.md
//! in the crate.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

cfg_if::cfg_if! {
    if #[cfg(feature = "vendored")] {
        pub use libftdi1_sys_vendored_lgpl::*;
    } else if #[cfg(feature = "bindgen")] {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    } else {
        include!("pregenerated.rs");
    }
}

#[cfg(test)]
mod test;
