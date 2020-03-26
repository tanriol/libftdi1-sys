use bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    #[cfg(not(windows))]
    pkg_config::Config::new().atleast_version("1.4").probe("libftdi1").unwrap();
    #[cfg(windows)]
    vcpkg::find_package("libftdi1").unwrap();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .default_enum_style(bindgen::EnumVariation::Rust{ non_exhaustive : true })
        .rustfmt_bindings(true)
        .whitelist_function("ftdi_.*") 
        .whitelist_type("ftdi_.*") 
        .whitelist_type("libusb_.*") 
        .blacklist_type("timeval")
        .blacklist_type("__.*")
        .raw_line("pub type timeval = libc::timeval;")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
