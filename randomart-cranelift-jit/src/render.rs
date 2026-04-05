use rayon::prelude::*;
use randomart_core::pixel_buffer::PixelBuffer;
use randomart_core::disable_ftz;

pub(crate) struct PixelCoordinates {
    pub x: f32,
    pub y: f32,
}

pub(crate) struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub(crate) fn render_pixels<F>(function: &F, width: u32, height: u32) -> PixelBuffer
where
    F: Sync + Fn(PixelCoordinates) -> Colour,
{
    const TILE_SIZE: u32 = 32;

    let tiles_x = (width + TILE_SIZE - 1) / TILE_SIZE;
    let tiles_y = (height + TILE_SIZE - 1) / TILE_SIZE;

    let tiles: Vec<(u32, u32)> = (0..tiles_y)
        .flat_map(|ty| (0..tiles_x).map(move |tx| (tx * TILE_SIZE, ty * TILE_SIZE)))
        .collect();

    // Each tile produces a vec of (global_x, global_y, r, g, b) tuples.
    rayon::broadcast(|_| unsafe { disable_ftz() });

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
                    let Colour { r, g, b } = function(PixelCoordinates { x, y });

                    let r = ((r + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
                    let g = ((g + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
                    let b = ((b + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
                    pixels.push((px, py, r, g, b));
                }
            }

            pixels
        })
        .collect();

    let mut buf = PixelBuffer::new(width, height);
    for tile in tile_pixels {
        for (x, y, r, g, b) in tile {
            buf.put_pixel(x, y, r, g, b);
        }
    }
    buf
}
