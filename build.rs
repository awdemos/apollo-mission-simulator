use std::env;
use std::path::PathBuf;

fn main() {
    let virtualagc_dir = PathBuf::from("/var/home/a/code/virtualagc");
    let lib_dir = virtualagc_dir.join("build");
    let include_dir = virtualagc_dir.join("yaAGC");

    cc::Build::new()
        .include(&include_dir)
        .file("src/virtual_agc_shim.c")
        .compile("virtual_agc_shim");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=yaAGC");
    println!("cargo:rerun-if-changed={}", lib_dir.join("libyaAGC.a").display());
}
