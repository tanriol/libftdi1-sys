use bindgen;
use std::env;
use std::path::PathBuf;

#[cfg(not(windows))]
fn main() {
    pkg_config::find_library("libftdi1").unwrap();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .default_enum_style(bindgen::EnumVariation::Rust{ non_exhaustive : true })
        .rustfmt_bindings(true)
        .blacklist_item("^_.*")
        .blacklist_item("^__.*")
        .blacklist_item("^[u]?int.*")
        .blacklist_item("^[U]?INT.*")
        .blacklist_item("^[A-Z].*_.*")
        .blacklist_type("time_t")
        .blacklist_type("timeval")
        .blacklist_type("timezone")
        .blacklist_type("itimerval")
        .blacklist_type("timespec")
        .blacklist_type("suseconds_t")
        .blacklist_type("fd_mask")
        .blacklist_type("fd_set")
        .blacklist_type("sigset_t")
        .blacklist_function(".*time.*")
        .blacklist_function("[p]?select")
        .opaque_type("libusb_*")
        .raw_line("pub type timeval = libc::timeval;")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(windows)]
fn main() {
    vcpkg::find_package("libftdi1").unwrap();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
