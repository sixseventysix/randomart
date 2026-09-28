use divan::{Bencher, black_box};
use randomart::{pcfg::OpGenerator, render::render, seed::derive_seeds};
use rayon::ThreadPoolBuilder;

fn main() {
    divan::main();
}

#[divan::bench(args = [1, 4, 10], sample_count = 20)]
fn vm(bencher: Bencher, threads: usize) {
    let pool = ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
    let channels = derive_seeds("hello world").map(|seed| OpGenerator::new(seed, 12).collect());
    bencher.bench_local(|| pool.install(|| render(black_box(&channels), 512, 512).unwrap()));
}
