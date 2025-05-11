use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use randomart::{
    utils::{fnv1a, render_pixels, PixelCoordinates, Colour},
    Grammar, Node, compile_node
};

fn bench_recursice_eval_rgb_pipeline(c: &mut Criterion) {
    c.bench_function("recursive_eval_rgb_pipeline", |b| {
        b.iter(|| {
            let string = "spiderman 1";
            let depth = 40;
            let width = 400;
            let height = 400;
            let seed = fnv1a(string);
            let mut grammar = Grammar::default(seed);

            let start_rule = 0;
            let mut generated_node = grammar.gen_rule(start_rule, depth).unwrap();
            generated_node.simplify_triple();

            let rgb_function = |coords: PixelCoordinates| {
                generated_node.eval_rgb(coords.x, coords.y)
            };

            black_box(render_pixels(rgb_function, width, height));
        });
    });
}

fn bench_compiled_closure_pipeline(c: &mut Criterion) {
    c.bench_function("compiled_closure_pipeline", |b| {
        b.iter(|| {
            let string = "spiderman 1";
            let depth = 40;
            let width = 400;
            let height = 400;
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

            black_box(render_pixels(rgb_function, width, height));
        });
    });
}

criterion_group!(
    benches_full_render,
    bench_recursice_eval_rgb_pipeline,
    bench_compiled_closure_pipeline
);

fn bench_compile_stage(c: &mut Criterion) {
    let string = "spiderman 1";
    let depth = 40;
    let _width = 400;
    let _height = 400;
    let seed = fnv1a(string);
    let mut grammar = Grammar::default(seed);

    let start_rule = 0;
    let mut generated_node = grammar.gen_rule(start_rule, depth).unwrap();
    generated_node.simplify_triple();
    c.bench_function("compile_stage", |b| {
        b.iter(|| {
            let (r_node, g_node, b_node) = match &*generated_node {
                Node::Triple(r, g, b) => (r, g, b),
                _ => panic!("Expected Triple node at the top level"),
            };

            let compiled = (
                compile_node(&*r_node),
                compile_node(&*g_node),
                compile_node(&*b_node),
            );

            black_box(compiled);
        });
    });
}

criterion_group!(
    benches_compile_stage,
    bench_compile_stage
);

fn bench_eval_rgb_loop(c: &mut Criterion) {
    let string = "spiderman 1";
    let depth = 40;
    let width = 400;
    let height = 400;

    let seed = fnv1a(string);
    let mut grammar = Grammar::default(seed);
    let start_rule = 0;
    let mut generated_node = grammar.gen_rule(start_rule, depth).unwrap();
    generated_node.simplify_triple();

    c.bench_function("eval_rgb_loop", |b| {
        b.iter(|| {
            let rgb_fn = |coords: PixelCoordinates| {
                generated_node.eval_rgb(coords.x, coords.y)
            };
            let _ = black_box(render_pixels(rgb_fn, width, height));
        });
    });
}

fn bench_compiled_eval_loop(c: &mut Criterion) {
    let string = "spiderman 1";
    let depth = 40;
    let width = 400;
    let height = 400;

    let seed = fnv1a(string);
    let mut grammar = Grammar::default(seed);
    let start_rule = 0;
    let mut generated_node = grammar.gen_rule(start_rule, depth).unwrap();
    generated_node.simplify_triple();

    c.bench_function("compiled_eval_loop", |b| {
        b.iter(|| {
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

            black_box(render_pixels(rgb_function, width, height));
        });
    });
}

criterion_group!(
    benches_hot_loops,
    bench_eval_rgb_loop,
    bench_compiled_eval_loop
);

criterion_main!(benches_compile_stage);