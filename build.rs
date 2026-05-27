use std::env;
use std::path::PathBuf;

fn main() {
    let virtualagc_dir = env::var("VIRTUALAGC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env::var("HOME").unwrap_or_else(|_| ".".to_string()))
                .join("code")
                .join("virtualagc")
        });

    let lib_dir = virtualagc_dir.join("build");
    let include_dir = virtualagc_dir.join("yaAGC");

    if !lib_dir.join("libyaAGC.a").exists() {
        panic!(
            "libyaAGC.a not found at {}. \
             Set VIRTUALAGC_DIR to the virtualagc repository root, \
             or build yaAGC first.",
            lib_dir.display()
        );
    }

    cc::Build::new()
        .include(&include_dir)
        .file("src/virtual_agc_shim.c")
        .compile("virtual_agc_shim");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=yaAGC");
    println!("cargo:rerun-if-changed={}", lib_dir.join("libyaAGC.a").display());
    println!("cargo:rerun-if-env-changed=VIRTUALAGC_DIR");
}
