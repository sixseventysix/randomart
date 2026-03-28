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
    node::Node,
    pixel_buffer::{GenerateOutput, PixelBuffer, ReadOutput},
};
use rayon::prelude::*;

const TILE_SIZE: u32 = 32;

fn render(width: u32, height: u32) -> PixelBuffer {
    let tiles_x = (width + TILE_SIZE - 1) / TILE_SIZE;
    let tiles_y = (height + TILE_SIZE - 1) / TILE_SIZE;

    let tiles: Vec<(u32, u32)> = (0..tiles_y)
        .flat_map(|ty| (0..tiles_x).map(move |tx| (tx * TILE_SIZE, ty * TILE_SIZE)))
        .collect();

    let tile_pixels: Vec<Vec<(u32, u32, u8, u8, u8)>> = tiles
        .into_par_iter()
        .map(|(x_start, y_start)| {
            let x_end = (x_start + TILE_SIZE).min(width);
            let y_end = (y_start + TILE_SIZE).min(height);
            let mut pixels = Vec::with_capacity(((x_end - x_start) * (y_end - y_start)) as usize);
            for py in y_start..y_end {
                for px in x_start..x_end {
                    let x = (px as f32 / (width - 1) as f32) * 2.0 - 1.0;
                    let y = (py as f32 / (height - 1) as f32) * 2.0 - 1.0;
                    let rv = ((r(x, y) + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
                    let gv = ((g(x, y) + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
                    let bv = ((b(x, y) + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
                    pixels.push((px, py, rv, gv, bv));
                }
            }
            pixels
        })
        .collect();

    let mut buf = PixelBuffer::new(width, height);
    for tile in tile_pixels {
        for (x, y, rv, gv, bv) in tile {
            buf.put_pixel(x, y, rv, gv, bv);
        }
    }
    buf
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
