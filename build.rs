// SPDX-License-Identifier: MIT

use std::env;
use std::path::PathBuf;

cfg_if::cfg_if! {
    if #[cfg(feature = "bindgen")] {
        use bindgen;
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
    if cfg!(feature = "vendored") {
        build_source();
    } else {
        if cfg!(feature = "libusb1-sys") {
            match env::var("DEP_USB_1.0_STATIC") {
                Ok(ref val) if val == "1" => {
                    panic!("libftdi1-sys can use static libusb1-sys with `vendored` feature only");
                }
                _ => {}
            }
        }

        if let Err(err) = find_library() {
            println!(
                "cargo:warning={}: {}",
                "Config for libftdi1 not found, falling back to default search path", err,
            );
            println!("cargo:rustc-link-lib=dylib=ftdi1");
        }
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "bindgen")] {
            fn bindings_builder() -> bindgen::Builder {
                bindgen::Builder::default()
                    .header(header_path())
                    .default_enum_style(bindgen::EnumVariation::NewType{ is_bitfield : false })
                    .rustfmt_bindings(true)
                    .whitelist_function("ftdi_.*")
                    .whitelist_type("ftdi_.*")
                    .no_copy("ftdi_eeprom")
                    .no_copy("ftdi_device_list")
                    .no_copy("ftdi_context")
                    .no_copy("ftdi_transfer_control")
                    .blacklist_type("timeval")
                    .blacklist_type("libusb_.*")
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

cfg_if::cfg_if! {
    if #[cfg(feature = "vendored")] {
        fn build_source() {
            let source = PathBuf::from(env::var("DEP_FTDI1_SOURCE_SOURCE_DIR")
                .expect("no source found"));

            let include_paths = link_and_get_include_paths();

            let mut libftdi = cc::Build::new();
            for path in include_paths {
                libftdi.include(path);
            }
            libftdi
                .files(&[source.join("ftdi.c"), source.join("ftdi_stream.c")])
                .include(source)
                .compile("ftdi");

        }

        fn header_path() -> String {
            let source = PathBuf::from(env::var("DEP_FTDI1_SOURCE_SOURCE_DIR")
                .expect("no source found"));
            source.join("ftdi.h").to_str().unwrap().to_owned()
        }
    } else {
        fn build_source() {}

        fn header_path() -> String {
            "wrapper.h".into()
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "libusb1-sys")] {
        fn link_and_get_include_paths() -> Vec<PathBuf> {
            vec![
                PathBuf::from(env::var("DEP_USB_1.0_INCLUDE")
                    .expect("libusb is required for libftdi"))
            ]
        }
    } else if #[cfg(all(windows, target_env="msvc"))] {
        fn link_and_get_include_paths() -> Vec<PathBuf> {
            let libusb = vcpkg::find_package("libusb-1.0")
                .expect("libusb is required for libftdi");
            libusb.include_paths
        }
    } else {
        fn link_and_get_include_paths() -> Vec<PathBuf> {
            let libusb = pkg_config::probe_library("libusb-1.0")
                .expect("libusb is required for libftdi");
            libusb.include_paths
        }
    }
}
