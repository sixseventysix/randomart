use criterion::{black_box, Criterion};
use criterion::{criterion_group, criterion_main};
use randomart::{ ClosureTree, Grammar };
use randomart::utils::{ render_pixels, PixelCoordinates, fnv1a };

pub fn bench_render_pixels(c: &mut Criterion) {
    let string = "spiderman 1";
    let depth = 40;
    let width = 400;
    let height = 400;

    let seed = fnv1a(&string);
    let mut grammar = Grammar::default(seed);

    let mut generated_node = grammar.gen_rule(0, depth).unwrap();
    generated_node.simplify_triple();

    let closure_tree = ClosureTree::from_node(&generated_node);

    let rgb_fn = move |coord: PixelCoordinates| closure_tree.eval_rgb(coord.x, coord.y);
    c.bench_function("render_pixels", |b| {
        b.iter(|| {
            black_box(render_pixels(&rgb_fn, width, height));
        });
    });
}

criterion_group!(benches, bench_render_pixels);
criterion_main!(benches);