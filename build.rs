#[cfg(not(windows))]
fn main() {
    pkg_config::find_library("libftdi1").unwrap();
}

#[cfg(windows)]
fn main() {
    vcpkg::find_package("libftdi1").unwrap();
}
