# `libftdi1-sys`
`libftdi1-sys` is a crate providing Rust bindings to the C library
[`libftdi1`](https://www.intra2net.com/en/developer/libftdi/index.php).

## Prerequisites
The libftdi1 bindings are generated at compile-time, per bindgen's
[recommended usage](https://rust-lang.github.io/rust-bindgen/library-usage.html).

* `libclang` must be installed and visible on your path. If you are using a
  `gcc`-toolchain and don't want to install the entirity of LLVM just for
  `libclang`,

* `libftdi` version 1.4 (the most recent version as of 2017) is required.

* The Minimum Supported Rust Version (MSRV) is stable `1.42`. However, older
  versions may work and simply have not been tested.
