use closure_tree::ClosureTree;
use engine::{backend::Backend, seed::generate_from_str};
use stack_vm::StackVm;
use xxhash_rust::xxh3::xxh3_64;

const GOLDEN: [(&str, u32, u64); 3] = [
    ("hello world", 8, 987757864781375965),
    ("randomart", 12, 13380684079561707144),
    ("colour", 16, 5108133957971540951),
];

fn backends() -> [(&'static str, &'static dyn Backend); 2] {
    [("closure-tree", &ClosureTree), ("stack-vm", &StackVm)]
}

#[test]
fn every_backend_matches_golden_hashes() {
    for (name, backend) in backends() {
        for (string, depth, expected) in GOLDEN {
            let channels = generate_from_str(string, depth);
            let buffer = backend.render(&channels, 128, 128).unwrap();
            assert_eq!(xxh3_64(&buffer.data), expected, "{name}: {string:?} at depth {depth}");
        }
    }
}
