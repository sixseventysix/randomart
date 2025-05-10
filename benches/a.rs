use criterion::{criterion_group, criterion_main, Criterion};
use randomart::{
    utils::{fnv1a, render_pixels, PixelCoordinates, Colour},
    Grammar, Node, compile_node
};

fn bench_old_recursive_pipeline(c: &mut Criterion) {
    let string = "spiderman 1";
    let depth = 40;
    let width = 400;
    let height = 400;

    c.bench_function("recursive_eval_rgb_pipeline", |b| {
        b.iter(|| {
            let seed = fnv1a(string);
            let mut grammar = Grammar::default(seed);

            let start_rule = 0;
            let mut generated_node = grammar.gen_rule(start_rule, depth).unwrap();
            generated_node.simplify_triple();

            let rgb_function = |coords: PixelCoordinates| {
                generated_node.eval_rgb(coords.x, coords.y)
            };

            let _img = render_pixels(rgb_function, width, height);
        });
    });
}

fn bench_compiled_closure_pipeline(c: &mut Criterion) {
    let string = "spiderman 1";
    let depth = 40;
    let width = 400;
    let height = 400;

    c.bench_function("compiled_closure_pipeline", |b| {
        b.iter(|| {
            let seed = fnv1a(string);
            let mut grammar = Grammar::default(seed);

            let start_rule = 0;
            let mut generated_node = grammar.gen_rule(start_rule, depth).unwrap();
            generated_node.simplify_triple();

            let (r_node, g_node, b_node) = match &*generated_node {
                Node::Triple(r, g, b) => (r, g, b),
                _ => panic!("Expected Triple node at the top level"),
            };

            let r_fn = compile_node(&*r_node);
            let g_fn = compile_node(&*g_node);
            let b_fn = compile_node(&*b_node);

            let rgb_function = move |coords: PixelCoordinates| Colour {
                r: r_fn(coords.x, coords.y),
                g: g_fn(coords.x, coords.y),
                b: b_fn(coords.x, coords.y),
            };

            let _img = render_pixels(rgb_function, width, height);
        });
    });
}

criterion_group!(
    benches,
    bench_old_recursive_pipeline,
    bench_compiled_closure_pipeline
);
criterion_main!(benches);