use divan::{AllocProfiler, Bencher, black_box};
use cpu::vm::compile;
use randomart::{pcfg::OpGenerator, pixel::pixel_position, seed::derive_seeds};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench(args = [8, 12, 16])]
fn compile_channels(bencher: Bencher, depth: u32) {
    let channels = derive_seeds("hello world").map(|seed| OpGenerator::new(seed, depth).collect::<Vec<_>>());
    let positions: Vec<f32> = (0..512).map(|pixel| pixel_position(pixel, 512)).collect();
    bencher.bench_local(|| black_box(&channels).each_ref().map(|ops| compile(ops, &positions, &positions)));
}
