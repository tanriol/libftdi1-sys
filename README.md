# `libftdi1-sys`
`libftdi1-sys` is a crate providing Rust bindings to the C library
[`libftdi1`](https://www.intra2net.com/en/developer/libftdi/index.php).

## Prerequisites
By default, the libftdi1 bindings are generated at compile-time, per bindgen's
[recommended usage](https://rust-lang.github.io/rust-bindgen/library-usage.html).

* `libclang` must be installed and visible on your path. If you are using a
  `gcc`-toolchain and don't want to install the entirity of LLVM just for
  `libclang`, you can use the following procedure (using a Debian-based
  ARM system as an example):

  ```
  sudo apt-get install libclang-dev
  export LIBCLANG_PATH=/usr/lib/llvm-7/lib
  export C_INCLUDE_PATH=/usr/lib/gcc/arm-linux-gnueabihf/8/include
  cargo build
  ```

* `libftdi` version 1.4 (the most recent version as of 2017) is required.

### MSRV
The Minimum Supported Rust Version (MSRV) is stable `1.34` when default
features are enabled (i.e. generating bindings at compile-time). If support for
older Rust versions is required, the pre-generated bindings
(down to `1.31`, Rust 2018) must be used instead.

## Pre-Generated Bindings
`libftdi1-sys` provides some bindings generated ahead-of-time from `bindgen`
as a fallback for users lacking `libclang`, requiring Rust 2018 support, or
otherwise. To use these pre-generated bindings, add the following to your
`Cargo.toml`:

```
[dependencies.libftdi1-sys]
default-features="false"
```

## Features
* `bindgen`: Generate bindings to `libftdi` at compile time.
  Enabled by default.
