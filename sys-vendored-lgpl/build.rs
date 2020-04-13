// SPDX-License-Identifier: MIT

use std::path::PathBuf;

fn main() {
    // Find include locations
    let include_paths = link_and_get_include_paths();

    let mut libftdi = cc::Build::new();
    for path in include_paths {
        libftdi.include(path);
    }
    libftdi
        .files(&["libftdi/src/ftdi.c", "libftdi/src/ftdi_stream.c"])
        .include(".")
        .compile("ftdi1-vendored");
}

cfg_if::cfg_if! {
    if #[cfg(all(windows, target_env="msvc"))] {
        fn link_and_get_include_paths() -> Vec<PathBuf> {
            let libusb = vcpkg::probe_library("libusb-1.0")
                .expect("libusb is required for libftdi");
            libusb.include_paths
        }
    } else {
        fn link_and_get_include_paths() -> Vec<PathBuf> {
            let libusb = pkg_config::probe_library("libusb-1.0")
                .expect("libusb is required for libftdi");
            libusb.include_paths
        }
    }
}
