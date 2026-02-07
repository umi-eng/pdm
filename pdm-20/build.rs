use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=memory.x");

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let mut linker_script = Vec::from(include_bytes!("memory.x"));
    writeln!(
        linker_script,
        "_HEADER_ALIGN = {:#x};",
        std::mem::align_of::<header::ImageHeader>(),
    )
    .unwrap();
    writeln!(
        linker_script,
        "_HEADER_SIZE = {:#x};",
        std::mem::size_of::<header::ImageHeader>()
    )
    .unwrap();
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(&linker_script)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
