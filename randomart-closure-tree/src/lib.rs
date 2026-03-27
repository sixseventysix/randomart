pub mod utils;

use utils::{compile_node, render_pixels, Colour, PixelCoordinates};
use randomart_core::{
    grammar::generate_tree_parallel,
    node::Node,
    pixel_buffer::{PixelBuffer, GenerateOutput, ReadOutput},
};
use xxhash_rust::xxh3::xxh3_64;

fn render_node(node: &Node, width: u32, height: u32) -> PixelBuffer {
    let (r, g, b) = match node {
        Node::Triple(r, g, b) => (r.as_ref(), g.as_ref(), b.as_ref()),
        _ => panic!("Expected top-level Triple node"),
    };
    let r_fn = compile_node(r);
    let g_fn = compile_node(g);
    let b_fn = compile_node(b);
    render_pixels(
        move |coord: PixelCoordinates| Colour {
            r: r_fn(coord.x, coord.y),
            g: g_fn(coord.x, coord.y),
            b: b_fn(coord.x, coord.y),
        },
        width,
        height,
    )
}

pub fn generate(string: &str, depth: u32, width: u32, height: u32) -> GenerateOutput {
    let seed: u64 = xxh3_64(string.as_bytes());
    let mut node = generate_tree_parallel(seed, depth).unwrap();
    node.simplify_triple();

    let json = serde_json::to_string_pretty(&*node).unwrap();
    let pixels = render_node(&node, width, height);
    GenerateOutput { pixels, json }
}

pub fn read_json(json: &str, width: u32, height: u32) -> ReadOutput {
    let node: Node = serde_json::from_str(json).expect("failed to deserialize node from JSON");
    let pixels = render_node(&node, width, height);
    ReadOutput { pixels }
}
