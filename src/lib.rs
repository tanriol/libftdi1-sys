//! libftdi1 is a library for working with FTDI chips like FT232BM, FT245BM,
//! FT2232C, FT2232D, FT245R, FT232H, FT230X The documentation for it is
//! available at http://www.intra2net.com/en/developer/libftdi/documentation/
//!
//! This wrapper is based on the rust-bindgen output for libftdi 1.4

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
