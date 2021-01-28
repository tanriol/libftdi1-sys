# `libftdi1-sys`
`libftdi1-sys` is a crate providing Rust bindings to the C library
[`libftdi1`](https://www.intra2net.com/en/developer/libftdi/index.php).

# Prerequisites
This crate requires `libftdi1` version 1.4 (August 2017) or later to be available as a system library
that can be found with `pkg-config` (everywhere except windows/MSVC) or `vcpkg` (windows/MSVC)
unless you activate the `vendored` feature.

By default the crate uses pregenerated bindings which should be fine in most cases.
In special cases the bindings can be regenerated using the `bindgen` feature.

Regenerating bindings has an additional requirement that 
  `libclang` must be installed and visible on your path. If you are using a
  `gcc`-toolchain and don't want to install the entirity of LLVM just for
  `libclang`, you can use the following procedure (using a Debian-based
  ARM system as an example):

  ```
  sudo apt-get install libclang-dev
  export LIBCLANG_PATH=/usr/lib/llvm-7/lib
  export C_INCLUDE_PATH=/usr/lib/gcc/arm-linux-gnueabihf/8/include
  cargo build
  ```

# MSRV
The Minimum Supported Rust Version (MSRV) is stable `1.34`.

# Features
* `libusb1-sys`: depend on `libusb1-sys` and use real `libusb` types instead of placeholders.
This makes it possible to interact directly with the underlying `libusb` structures.
* `vendored`: build a custom copy of `libftdi` instead of using the system one.
Note that this includes LGPL code in your build.
* `bindgen`: Generate bindings to `libftdi` at compile time.

# Contributing

This crate is expected to be passively maintained, not actively updated.
That being said, if you want to update binding generation,
you can set the `LIBFTDI1_SYS_DEVEL` environment variable with any value
so that an updated binding file is written in `src` alongside the original one
enabling you to compare them and copy it over the old one to update.
