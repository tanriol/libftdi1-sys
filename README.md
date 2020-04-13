# `libftdi1-sys`
`libftdi1-sys` is a crate providing Rust bindings to the C library
[`libftdi1`](https://www.intra2net.com/en/developer/libftdi/index.php).

# Prerequisites
This crate requires `libftdi1` version 1.4 (August 2017) or later to be available as a system library
that can be found with `pkg-config` (everywhere except windows/MSVC) or `vcpkg` (windows/MSVC).

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
The Minimum Supported Rust Version (MSRV) is stable `1.31` (Rust 2018) with pregenerated bindings,
or `1.34` with the `bindgen` feature.

# Features
* `bindgen`: Generate bindings to `libftdi` at compile time.
  Enabled by default.
