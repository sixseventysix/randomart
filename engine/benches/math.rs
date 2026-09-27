use divan::{Bencher, black_box};
use engine::{ftz::disable_ftz, math, render::pixel_position};

const PIXELS: usize = 512 * 512;

fn main() {
    divan::main();
}

fn inputs() -> Vec<f32> {
    (0..PIXELS).map(|i| pixel_position(i, PIXELS as u32)).collect()
}

fn run(bencher: Bencher, f: fn(f32) -> f32) {
    disable_ftz();
    let xs = inputs();
    bencher.bench_local(|| {
        for &x in black_box(&xs) {
            black_box(f(x));
        }
    });
}

#[divan::bench]
fn sinf(bencher: Bencher) {
    run(bencher, math::sinf)
}

#[divan::bench]
fn cosf(bencher: Bencher) {
    run(bencher, math::cosf)
}

#[divan::bench]
fn expf(bencher: Bencher) {
    run(bencher, math::expf)
}

#[divan::bench]
fn sqrtf(bencher: Bencher) {
    run(bencher, math::sqrtf)
}
