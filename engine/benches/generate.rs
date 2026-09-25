use engine::{op::Op, seed::generate_from_str};

fn main() {
    divan::main();
}

#[divan::bench(args = [8, 12, 16])]
fn generate(depth: u32) -> [Vec<Op>; 3] {
    generate_from_str(divan::black_box("hello world"), depth)
}
