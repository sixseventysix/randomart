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

    let sinf_row = manifest_dir.join("core-math-rows/sinf_row.c");
    let cosf_row = manifest_dir.join("core-math-rows/cosf_row.c");
    let expf_row = manifest_dir.join("core-math-rows/expf_row.c");

    for path in [&sinf, &cosf, &expf, &sinf_row, &cosf_row, &expf_row] {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    cc::Build::new()
        .file(sinf_row)
        .file(cosf_row)
        .file(expf_row)
        .flag_if_supported("-std=c11")
        .compile("core_math");
}
