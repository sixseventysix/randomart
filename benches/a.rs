use criterion::{criterion_group, criterion_main, Criterion};
use randomart_core::utils::{render_pixels, PixelCoordinates, Colour};
include!(concat!(env!("OUT_DIR"), "/generated_rgb_fn.rs"));

pub fn bench_render_pixels(c: &mut Criterion) {
    let width = 400;
    let height = 400;

    let r = r_fn();
    let g = g_fn();
    let b = b_fn();

    let rgb_function = move |coord: PixelCoordinates| Colour {
        r: r(coord.x, coord.y),
        g: g(coord.x, coord.y),
        b: b(coord.x, coord.y),
    };

    c.bench_function("render_pixels", |b| {
        b.iter(|| {
            let _img = std::hint::black_box(render_pixels(&rgb_function, width, height));
        });
    });
}

criterion_group!(benches, bench_render_pixels);
criterion_main!(benches);