# 1.1.3 (2023-12-14)

Changed:

- use the rustonomicon-recommended extern type pattern without `libusb1-sys` (#19)

# 1.1.2 (2022-02-13)

Changed:

- allow `libusb1-sys` versions up to `0.6.*`

# 1.1.1 (2021-08-12)

Fixed:

- fix `bindgen` feature compatibility with `libftdi` before 1.5

# 1.1.0 (2021-08-12)

BREAKING:

Changed the SIO constants used for flow control modes to be signed.
Their only real use is with the matching function, which takes `c_int`,
while they used to be `u32`. Formally speaking this is a breaking change, but:
- I've manually verified all `crates.io` revdeps to be okay with that (they don't use them);
- this change removes a mandatory and completely useless type conversion.

Changed:

- allow `libusb` versions up to `0.5.*`
- bump dependencies on `cfg-if` and `bindgen`
  - as a side effect, enums now use the `c_uint` type, not `u32` directly
- deprecate `SIO_RESET_PURGE_{RX,TX}` (deprecated in `libftdi` 1.5)
