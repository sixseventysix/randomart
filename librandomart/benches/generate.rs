use randomart::{op::Op, pcfg::OpGenerator, seed::derive_seeds};

fn main() {
    divan::main();
}

#[divan::bench(args = [8, 12, 16])]
fn generate(depth: u32) -> [Vec<Op>; 3] {
    derive_seeds(divan::black_box("hello world")).map(|seed| OpGenerator::new(seed, depth).collect())
}
