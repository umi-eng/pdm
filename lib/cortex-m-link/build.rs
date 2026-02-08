use std::io::Write;
use std::{fs::File, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=custom.x");

    let out = &PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    let mut script = include_bytes!("custom.x").to_vec();
    writeln!(
        script,
        "\n/* Variables provided by cortex-m-link/build.rs */"
    )
    .unwrap();
    writeln!(
        script,
        "_HEADER_ALIGN = {:#x};",
        std::mem::align_of::<header::ImageHeader>(),
    )
    .unwrap();
    writeln!(
        script,
        "_HEADER_SIZE = {:#x};",
        std::mem::size_of::<header::ImageHeader>()
    )
    .unwrap();
    File::create(out.join("custom.x"))
        .unwrap()
        .write_all(&script)
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());
}
