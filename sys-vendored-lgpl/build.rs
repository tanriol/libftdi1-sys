// SPDX-License-Identifier: MIT

fn main() {
    // Find include locations
    let libusb = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("libusb-1.0")
        .expect("libusb is required for libftdi");

    let mut libftdi = cc::Build::new();
    for path in libusb.include_paths {
        libftdi.include(path);
    }
    libftdi
        .files(&["libftdi/src/ftdi.c", "libftdi/src/ftdi_stream.c"])
        .include(".")
        .compile("ftdi1-vendored");

    // Add the library dependency
    // Note that this has to be done after compiling to get correct linking order
    pkg_config::probe_library("libusb-1.0").expect("libusb is required for libftdi");
}
