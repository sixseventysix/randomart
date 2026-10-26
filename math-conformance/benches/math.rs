use divan::Bencher;
use math_conformance::{core_math_cosf_row, core_math_expf_row, core_math_sinf_row};
use cpu::{ftz::disable_ftz, render::pixel_position};

const PIXELS: usize = 512 * 512;

fn main() {
    divan::main();
}

fn run(bencher: Bencher, f: impl Fn(&mut [f32])) {
    disable_ftz();
    let xs: Vec<f32> = (0..PIXELS).map(|i| pixel_position(i, PIXELS as u32)).collect();
    bencher.with_inputs(|| xs.clone()).bench_local_refs(|values| f(values));
}

fn scalar(bencher: Bencher, f: impl Fn(f32) -> f32) {
    run(bencher, |values| {
        for v in values {
            *v = f(*v);
        }
    });
}

mod sinf {
    use super::*;

    #[divan::bench]
    fn core_math(bencher: Bencher) {
        run(bencher, core_math_sinf_row)
    }

    #[divan::bench]
    fn math(bencher: Bencher) {
        scalar(bencher, ::math::sinf)
    }
}

mod cosf {
    use super::*;

    #[divan::bench]
    fn core_math(bencher: Bencher) {
        run(bencher, core_math_cosf_row)
    }

    #[divan::bench]
    fn math(bencher: Bencher) {
        scalar(bencher, ::math::cosf)
    }
}

mod expf {
    use super::*;

    #[divan::bench]
    fn core_math(bencher: Bencher) {
        run(bencher, core_math_expf_row)
    }

    #[divan::bench]
    fn math(bencher: Bencher) {
        scalar(bencher, ::math::expf)
    }
}
