// SPDX-License-Identifier: LGPL-2.1

use std::{fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version_string = format!(
        "{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
    );
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let target_dir = out_dir.join("source");
    let _ = fs::remove_dir_all(&target_dir);
    fs::create_dir(&target_dir)?;
    for entry in fs::read_dir("libftdi/src")? {
        let entry = entry?;
        if entry.file_name() != "ftdi_version_i.h.in" {
            fs::copy(entry.path(), target_dir.join(entry.file_name()))?;
        } else {
            let text = fs::read_to_string(entry.path())?;
            let rendered = text
                .replace("@MAJOR_VERSION@", env!("CARGO_PKG_VERSION_MAJOR"))
                .replace("@MINOR_VERSION@", env!("CARGO_PKG_VERSION_MINOR"))
                .replace("@VERSION_STRING@", &version_string)
                .replace("@SNAPSHOT_VERSION@", "");
            fs::write(target_dir.join("ftdi_version_i.h"), rendered)?;
        }
    }
    println!("cargo:source-dir={}", target_dir.to_str().unwrap());

    Ok(())
}
