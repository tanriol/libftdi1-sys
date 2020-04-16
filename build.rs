// SPDX-License-Identifier: MIT

cfg_if::cfg_if! {
    if #[cfg(feature = "bindgen")] {
        use bindgen;

        use std::env;
        use std::path::PathBuf;
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(windows, target_env="msvc"))] {
        fn find_library() -> Result<vcpkg::Library, vcpkg::Error> {
            vcpkg::find_package("libftdi1")
        }
    } else {
        fn find_library() -> Result<pkg_config::Library, pkg_config::Error> {
            pkg_config::Config::new().atleast_version("1.4").probe("libftdi1")
        }
    }
}

fn main() {
    if let Err(err) = find_library() {
        println!(
            "cargo:warning={}: {}",
            "Config for libftdi1 not found, falling back to default search path",
            err,
        );
        println!("cargo:rustc-link-lib=dylib=ftdi1");
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "bindgen")] {
            fn bindings_builder() -> bindgen::Builder {
                bindgen::Builder::default()
                    .header("wrapper.h")
                    .default_enum_style(bindgen::EnumVariation::NewType{ is_bitfield : false })
                    .rustfmt_bindings(true)
                    .whitelist_function("ftdi_.*")
                    .whitelist_type("ftdi_.*")
                    .no_copy("libusb_.*")
                    .no_copy("ftdi_eeprom")
                    .no_copy("ftdi_device_list")
                    .no_copy("ftdi_context")
                    .no_copy("ftdi_transfer_control")
                    .blacklist_type("timeval")
                    .blacklist_type("__.*")
                    .raw_line("pub type timeval = libc::timeval;")
            }

            let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
            bindings_builder()
                .generate()
                .expect("Unable to generate bindings")
                .write_to_file(out_path.join("bindings.rs"))
                .expect("Couldn't write bindings!");

            println!("cargo:rerun-if-env-changed=LIBFTDI1_SYS_DEVEL");
            if env::var("LIBFTDI1_SYS_DEVEL").is_ok() {
                bindings_builder()
                    .layout_tests(false)
                    .generate()
                    .expect("Unable to generated bindings without tests")
                    .write_to_file("src/pregenerated.rs.updated")
                    .expect("Couldn't update pregenerated bindings!");
            }
        } else {
            // Anything not depending on bindgen feature goes here.
        }
    }
}
