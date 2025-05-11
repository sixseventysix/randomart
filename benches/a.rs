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

    let (r_node, g_node, b_node) = match &*generated_node {
        randomart::Node::Triple(r, g, b) => (r, g, b),
            _ => panic!("Expected Node::Triple at the top level"),
    };

    let mut r_instructions = Vec::new();
    let mut g_instructions = Vec::new();
    let mut b_instructions = Vec::new();

    randomart::emit_postfix(r_node, &mut r_instructions);
    randomart::emit_postfix(g_node, &mut g_instructions);
    randomart::emit_postfix(b_node, &mut b_instructions);

    let program = randomart::PostfixRgbProgram {
        r: r_instructions,
        g: g_instructions,
        b: b_instructions,
    };
    let rgb_fn = program.to_fn();
    c.bench_function("render_pixels", |b| {
        b.iter(|| {
            black_box(render_pixels(&rgb_fn, width, height));
        });
    });
}

criterion_group!(benches, bench_render_pixels);
criterion_main!(benches);