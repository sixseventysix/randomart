use std::path::{Path, PathBuf};

fn core_math_dir(manifest_dir: &Path) -> PathBuf {
    manifest_dir.join("..").join("core-math")
}

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let core_math = core_math_dir(&manifest_dir);

    let sinf = core_math.join("src/binary32/sin/sinf.c");
    let cosf = core_math.join("src/binary32/cos/cosf.c");
    let expf = core_math.join("src/binary32/exp/expf.c");

    println!("cargo:rerun-if-changed={}", sinf.display());
    println!("cargo:rerun-if-changed={}", cosf.display());
    println!("cargo:rerun-if-changed={}", expf.display());

    cc::Build::new()
        .file(sinf)
        .file(cosf)
        .file(expf)
        .flag_if_supported("-std=c11")
        .compile("core_math");
}
