mod jit;
mod render;

use crate::{
    render::{render_pixels, PixelCoordinates, Colour},
    jit::build_jit_function_triple,
};
use randomart_core::{
    grammar::generate_tree_parallel,
    node::Node,
    pixel_buffer::{GenerateOutput, ReadOutput},
};
use xxhash_rust::xxh3::xxh3_64;

pub fn generate(string: &str, depth: u32, width: u32, height: u32) -> GenerateOutput {
    let seed: u64 = xxh3_64(string.as_bytes());
    let mut node = generate_tree_parallel(seed, depth).unwrap();
    node.simplify_triple();

    let json = serde_json::to_string_pretty(&*node).unwrap();

    let (r_jit_fn, g_jit_fn, b_jit_fn) = build_jit_function_triple(&node);
    let rgb_fn = |coord: PixelCoordinates| Colour {
        r: r_jit_fn(coord.x, coord.y),
        g: g_jit_fn(coord.x, coord.y),
        b: b_jit_fn(coord.x, coord.y),
    };

    let pixels = render_pixels(&rgb_fn, width, height);
    GenerateOutput { pixels, json }
}

pub fn read_json(json: &str, width: u32, height: u32) -> ReadOutput {
    let node: Node = serde_json::from_str(json).expect("failed to deserialize node from JSON");

    let (r_jit_fn, g_jit_fn, b_jit_fn) = build_jit_function_triple(&node);
    let rgb_fn = |coord: PixelCoordinates| Colour {
        r: r_jit_fn(coord.x, coord.y),
        g: g_jit_fn(coord.x, coord.y),
        b: b_jit_fn(coord.x, coord.y),
    };

    let pixels = render_pixels(&rgb_fn, width, height);
    ReadOutput { pixels }
}
