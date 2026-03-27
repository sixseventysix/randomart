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
