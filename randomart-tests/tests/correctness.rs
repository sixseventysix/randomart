#[test]
fn jit_matches_closure_tree() {
    let jit = randomart_cranelift_jit::generate("test", 8, 64, 64);
    let closure = randomart_closure_tree::generate("test", 8, 64, 64);
    assert_eq!(jit.pixels, closure.pixels);
}

#[test]
fn jit_matches_closure_tree_spiderman2_depth30() {
    let jit = randomart_cranelift_jit::generate("spiderman 2", 30, 512, 512);
    let closure = randomart_closure_tree::generate("spiderman 2", 30, 512, 512);
    assert_eq!(jit.pixels, closure.pixels);
}

#[test]
fn aot_matches_closure_tree() {
    let seed = randomart_llvm_aot::baked_seed();
    let depth = randomart_llvm_aot::baked_depth();
    let aot = randomart_llvm_aot::generate(seed, depth, 64, 64);
    let closure = randomart_closure_tree::generate(seed, depth, 64, 64);
    assert_eq!(aot.pixels, closure.pixels);
}

#[cfg(target_os = "macos")]
#[test]
#[ignore = "Metal backend is expected to fail because it doesn't use CORE-MATH"]
fn metal_matches_closure_tree() {
    let metal = match std::panic::catch_unwind(|| randomart_metal::generate("test", 8, 64, 64)) {
        Ok(output) => output,
        Err(_) => {
            eprintln!("Skipping Metal test: no Metal device or Metal init failed.");
            return;
        }
    };
    let closure = randomart_closure_tree::generate("test", 8, 64, 64);
    assert_eq!(metal.pixels, closure.pixels);
}
