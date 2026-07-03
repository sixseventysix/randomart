mod jit;

use crate::jit::build_jit_function_triple;
use randomart_core::{
    grammar::generate_tree_parallel,
    node::Node,
    pixel_buffer::{GenerateOutput, ReadOutput},
    render::{render_tiled, Colour, PixelCoordinates},
};
use anyhow::{bail, Context, Result};
use xxhash_rust::xxh3::xxh3_64;

pub fn generate(string: &str, depth: u32, width: u32, height: u32) -> Result<GenerateOutput> {
    let seed: u64 = xxh3_64(string.as_bytes());
    let mut node = generate_tree_parallel(seed, depth)
        .context("tree generation failed")?;
    node.simplify_triple();

    let json = serde_json::to_string_pretty(&*node)
        .context("failed to serialize node tree")?;

    let (r_jit_fn, g_jit_fn, b_jit_fn) = build_jit_function_triple(&node);
    let rgb_fn = |coord: PixelCoordinates| Colour {
        r: r_jit_fn(coord.x, coord.y),
        g: g_jit_fn(coord.x, coord.y),
        b: b_jit_fn(coord.x, coord.y),
    };

    let pixels = render_tiled(&rgb_fn, width, height);
    Ok(GenerateOutput { pixels, json })
}

pub fn read_json(json: &str, width: u32, height: u32) -> Result<ReadOutput> {
    let node: Node = serde_json::from_str(json)
        .context("failed to deserialize node tree from JSON")?;

    if !matches!(node, Node::Triple(_, _, _)) {
        bail!("top-level JSON node must be a Triple");
    }

    let (r_jit_fn, g_jit_fn, b_jit_fn) = build_jit_function_triple(&node);
    let rgb_fn = |coord: PixelCoordinates| Colour {
        r: r_jit_fn(coord.x, coord.y),
        g: g_jit_fn(coord.x, coord.y),
        b: b_jit_fn(coord.x, coord.y),
    };

    let pixels = render_tiled(&rgb_fn, width, height);
    Ok(ReadOutput { pixels })
}
