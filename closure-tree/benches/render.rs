use closure_tree::ClosureTree;
use divan::{Bencher, black_box};
use engine::{backend::Backend, seed::generate_from_str};
use rayon::ThreadPoolBuilder;

fn main() {
    divan::main();
}

#[divan::bench(args = [1, 4, 10], sample_count = 20)]
fn render(bencher: Bencher, threads: usize) {
    let pool = ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
    let channels = generate_from_str("hello world", 12);
    bencher.bench_local(|| {
        pool.install(|| ClosureTree.render(black_box(&channels), 512, 512).unwrap())
    });
}
