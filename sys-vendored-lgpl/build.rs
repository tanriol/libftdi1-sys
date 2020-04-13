// SPDX-License-Identifier: MIT

fn main() {
    // Find include locations
    let libusb = pkg_config::Config::new()
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
}
