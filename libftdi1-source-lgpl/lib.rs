// SPDX-License-Identifier: LGPL-2.1

//! This crate provides the LGPL source of `libftdi` for `libftdi1-sys[vendored]`.
//! If your dependency tree includes this crate, your final binary likely includes LGPL-licensed code.
//! If you're not sure whether that's acceptable for your use case, please consult your lawyer.

/// Reports the version of `libftdi` this package provides.
pub fn libftdi_version() -> String {
    return format!(
        "{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
    );
}
