// SPDX-License-Identifier: MIT

//! libftdi1 is a library for working with FTDI chips like FT232BM, FT245BM,
//! FT2232C, FT2232D, FT245R, FT232H, FT230X. The documentation for it is
//! available [upstream].
//!
//! [upstream]: http://www.intra2net.com/en/developer/libftdi/documentation/
//!
//! This wrapper was generated using rust-bindgen.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::libusb1_sys::{libusb_device, libusb_context, libusb_device_handle, libusb_transfer};

cfg_if::cfg_if! {
    if #[cfg(feature = "bindgen")] {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    } else {
        include!("pregenerated.rs");
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "libusb1-sys")] {
        pub use libusb1_sys;
    } else {
        /// Internal placeholders for `libusb` types. Do not use externally.
        mod libusb1_sys {
            #[repr(C)]
            pub struct libusb_transfer {
                _address: u8,
            }
            #[repr(C)]
            pub struct libusb_context {
                _address: u8,
            }
            #[repr(C)]
            pub struct libusb_device_handle {
                _address: u8,
            }
            #[repr(C)]
            pub struct libusb_device {
                _address: u8,
            }
        }
    }
}
