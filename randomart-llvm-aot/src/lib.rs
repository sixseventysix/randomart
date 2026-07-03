include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub fn baked_seed() -> &'static str {
    option_env!("RANDOMART_SEED").unwrap_or("default")
}

pub fn baked_depth() -> u32 {
    option_env!("RANDOMART_DEPTH")
        .and_then(|v| v.parse().ok())
        .unwrap_or(8)
}

use randomart_core::{
    grammar::generate_tree_parallel,
    pixel_buffer::{GenerateOutput, PixelBuffer, ReadOutput},
    render::{render_tiled, Colour, PixelCoordinates},
};

fn render(width: u32, height: u32) -> PixelBuffer {
    render_tiled(
        &|coord: PixelCoordinates| Colour {
            r: r(coord.x, coord.y),
            g: g(coord.x, coord.y),
            b: b(coord.x, coord.y),
        },
        width,
        height,
    )
}

/// Renders the expression baked in at compile time.
/// `string` and `depth` are ignored at runtime — they were consumed by build.rs.
pub fn generate(_string: &str, _depth: u32, width: u32, height: u32) -> GenerateOutput {
    use xxhash_rust::xxh3::xxh3_64;
    let seed_str = option_env!("RANDOMART_SEED").unwrap_or("default");
    let depth_str: u32 = option_env!("RANDOMART_DEPTH")
        .and_then(|v| v.parse().ok())
        .unwrap_or(8);
    let seed = xxh3_64(seed_str.as_bytes());
    let mut node = generate_tree_parallel(seed, depth_str).unwrap();
    node.simplify_triple();
    let json = serde_json::to_string_pretty(&*node).unwrap();
    let pixels = render(width, height);
    GenerateOutput { pixels, json }
}

pub fn read_json(_json: &str, width: u32, height: u32) -> ReadOutput {
    let pixels = render(width, height);
    ReadOutput { pixels }
}
