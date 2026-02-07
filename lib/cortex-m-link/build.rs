use std::io::Write;
use std::{fs::File, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=custom.x");

    let out = &PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    File::create(out.join("custom.x"))
        .unwrap()
        .write_all(include_bytes!("custom.x"))
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());
}
