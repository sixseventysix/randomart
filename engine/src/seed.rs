use crate::op::Op;
use crate::pcfg::Grammar;
use xxhash_rust::xxh3::xxh3_64;

pub fn derive_seeds(base: u64) -> [u64; 3] {
    let s = base.to_le_bytes();
    [
        xxh3_64(&[s.as_slice(), b"-a"].concat()),
        xxh3_64(&[s.as_slice(), b"-b"].concat()),
        xxh3_64(&[s.as_slice(), b"-c"].concat()),
    ]
}

pub fn generate_from_str(string: &str, depth: u32) -> [Vec<Op>; 3] {
    derive_seeds(xxh3_64(string.as_bytes())).map(|seed| Grammar::new(seed, depth).collect())
}
