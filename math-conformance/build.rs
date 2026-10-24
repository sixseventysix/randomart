use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let core_math = manifest_dir.join("../core-math/src/binary32");
    let mut build = cc::Build::new();

    for (dir, name) in [("sin", "sinf"), ("cos", "cosf"), ("exp", "expf")] {
        let source = core_math.join(dir).join(format!("{name}.c"));
        let row = manifest_dir.join("core-math-rows").join(format!("{name}_row.c"));
        println!("cargo:rerun-if-changed={}", source.display());
        println!("cargo:rerun-if-changed={}", row.display());
        build.file(row);
    }

    build.flag_if_supported("-std=c11").compile("core_math");
}
