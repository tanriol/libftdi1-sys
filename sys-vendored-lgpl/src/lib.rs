// SPDX-License-Identifier: MIT

//! *Attention*: this is an internal crate, you should depend on `libftdi1-sys` instead!
//!
//! libftdi1 is a library for working with FTDI chips like FT232BM, FT245BM,
//! FT2232C, FT2232D, FT245R, FT232H, FT230X The documentation for it is
//! available at http://www.intra2net.com/en/developer/libftdi/documentation/
//!
//! This crate includes the LGPL code for libftdi1 (the vendored copy that `libftdi1-sys[vendored]`
//! uses and thus is licensed under LGPL 2.1. If you're seeing this crate in your dependency
//! tree, this means that LGPL 2.1 licensed code will be compiled into your executable.
//!
//! For more detail see the README.md file in this crate and/or its git repository.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("pregenerated.rs");
