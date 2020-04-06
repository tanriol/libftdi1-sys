# About

This crate contains and builds the vendored copy of `libftdi1` for use by `libftdi1-sys`.
Note that it does not provide any bindings on its own, but leaves that to `libftdi1-sys`.

The reason this crate exists is that `libftdi1` primary library components are licensed under `LGPL-2.1`, unlike `libftdi1-sys`, which is under `MIT OR Apache-2.0`.
Thus they are separated into two different crates that have different licenses to make this obvious to license checking tools.

If you see this crate in your dependency tree, LGPL 2.1 licensed code will be linked into the final executable.
If you're not sure whether that's fine for your purpose, please see the `GNU LGPL 2.1` license text [GNU LGPL 2.1][online] or in the `lgpl-2.1.txt` file in this crate and/or consult a lawyer.

[GNU LGPL 2.1]: https://www.gnu.org/licenses/old-licenses/lgpl-2.1.txt

# Rust version support

This crate supports all Rust versions that support Rust 2018 edition (i.e. 1.31.0 and higher).

# Maintenance

This crate is passively maintained. There are a coulpe directions for possible contributions:
- support for building in non-Linux environments
- cross-compilation support

If a new upstream version is released, the following steps are needed to update:
- update the submodule to the new tag
- update the `ftdi_version_i.h` header file in the crate root based on the upstream template
