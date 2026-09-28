use randomart::randomart;
use xxhash_rust::xxh3::xxh3_64;

const GOLDEN: [(&str, u32, u64); 3] = [
    ("hello world", 8, 987757864781375965),
    ("randomart", 12, 13380684079561707144),
    ("colour", 16, 5108133957971540951),
];

#[test]
fn matches_golden_hashes() {
    for (string, depth, expected) in GOLDEN {
        let buffer = randomart(string, depth, 128, 128).unwrap();
        assert_eq!(xxh3_64(&buffer.data), expected, "{string:?} at depth {depth}");
    }
}
