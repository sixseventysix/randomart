use closure_tree::ClosureTree;
use divan::{Bencher, black_box};
use engine::{backend::Backend, seed::generate_from_str};
use rayon::ThreadPoolBuilder;
use stack_vm::{MemoStackVm, StackVm};

fn main() {
    divan::main();
}

fn run(bencher: Bencher, threads: usize, backend: &(dyn Backend + Sync)) {
    let pool = ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
    let channels = generate_from_str("hello world", 12);
    bencher.bench_local(|| {
        pool.install(|| backend.render(black_box(&channels), 512, 512).unwrap())
    });
}

#[divan::bench(args = [1, 4, 10], sample_count = 20)]
fn closure_tree(bencher: Bencher, threads: usize) {
    run(bencher, threads, &ClosureTree)
}

#[divan::bench(args = [1, 4, 10], sample_count = 20)]
fn stack_vm(bencher: Bencher, threads: usize) {
    run(bencher, threads, &StackVm)
}

#[divan::bench(args = [1, 4, 10], sample_count = 20)]
fn memo_stack_vm(bencher: Bencher, threads: usize) {
    run(bencher, threads, &MemoStackVm)
}
