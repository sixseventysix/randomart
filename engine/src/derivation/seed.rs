use crate::derivation::walk::Walk;
use crate::grammar::Grammar;
use crate::tree::node::Node;
use xxhash_rust::xxh3::xxh3_64;

pub fn derive_seeds(base: u64) -> (u64, u64, u64) {
    let s = base.to_le_bytes();
    (
        xxh3_64(&[s.as_slice(), b"-a"].concat()),
        xxh3_64(&[s.as_slice(), b"-b"].concat()),
        xxh3_64(&[s.as_slice(), b"-c"].concat()),
    )
}

pub fn generate_tree_parallel(grand_seed: u64, depth: u32) -> Option<Box<Node>> {
    let (seed_a, seed_b, seed_c) = derive_seeds(grand_seed);

    let grammar = Grammar::default();

    let (b, c) = rayon::join(
        || Walk::new(&grammar, seed_b).gen_rule(1, depth - 1),
        || Walk::new(&grammar, seed_c).gen_rule(1, depth - 1),
    );

    let a = Walk::new(&grammar, seed_a).gen_rule(1, depth - 1);

    match (a, b, c) {
        (Some(a), Some(b), Some(c)) => Some(Box::new(Node::Triple(a, b, c))),
        _ => None,
    }
}

pub fn generate_from_str(string: &str, depth: u32) -> Option<Box<Node>> {
    let mut node = generate_tree_parallel(xxh3_64(string.as_bytes()), depth)?;
    node.simplify_triple();
    Some(node)
}
