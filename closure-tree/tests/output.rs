use closure_tree::ClosureTree;
use engine::{backend::Backend, seed::generate_from_str};
use xxhash_rust::xxh3::xxh3_64;

fn render_hash(string: &str, depth: u32) -> u64 {
    let channels = generate_from_str(string, depth);
    let buffer = ClosureTree.render(&channels, 128, 128).unwrap();
    xxh3_64(&buffer.data)
}

#[test]
fn hello_world_depth_8() {
    assert_eq!(render_hash("hello world", 8), 987757864781375965);
}

#[test]
fn randomart_depth_12() {
    assert_eq!(render_hash("randomart", 12), 13380684079561707144);
}

#[test]
fn colour_depth_16() {
    assert_eq!(render_hash("colour", 16), 5108133957971540951);
}
