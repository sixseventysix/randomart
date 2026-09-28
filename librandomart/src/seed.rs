use xxhash_rust::xxh3::xxh3_64;

pub fn derive_seeds(text: &str) -> [u64; 3] {
    let s = xxh3_64(text.as_bytes()).to_le_bytes();
    [
        xxh3_64(&[s.as_slice(), b"-a"].concat()),
        xxh3_64(&[s.as_slice(), b"-b"].concat()),
        xxh3_64(&[s.as_slice(), b"-c"].concat()),
    ]
}
