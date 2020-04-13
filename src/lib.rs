// SPDX-License-Identifier: MIT

//! libftdi1 is a library for working with FTDI chips like FT232BM, FT245BM,
//! FT2232C, FT2232D, FT245R, FT232H, FT230X The documentation for it is
//! available at http://www.intra2net.com/en/developer/libftdi/documentation/
//!
//! This wrapper was generated using rust-bindgen for libftdi 1.4.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

cfg_if::cfg_if! {
    if #[cfg(feature = "bindgen")] {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    } else {
        include!("pregenerated.rs");
    }
}
