use image::{RgbImage, Rgb};
use rayon::prelude::*;

pub(crate) struct PixelCoordinates {
    pub x: f32,
    pub y: f32
}

pub(crate) struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

pub(crate) fn render_pixels<F>(function: &F, width: u32, height: u32) -> RgbImage
where
    F: Sync + Fn(PixelCoordinates) -> Colour,
{
    const TILE_SIZE: u32 = 32;

    let tiles_x = (width + TILE_SIZE - 1) / TILE_SIZE;
    let tiles_y = (height + TILE_SIZE - 1) / TILE_SIZE;

    let tiles: Vec<(u32, u32)> = (0..tiles_y)
        .flat_map(|ty| (0..tiles_x).map(move |tx| (tx * TILE_SIZE, ty * TILE_SIZE)))
        .collect();

    let partial_tiles: Vec<(u32, u32, RgbImage)> = tiles
        .into_par_iter()
        .map(|(x_start, y_start)| {
            let x_end = (x_start + TILE_SIZE).min(width);
            let y_end = (y_start + TILE_SIZE).min(height);
            let tile_width = x_end - x_start;
            let tile_height = y_end - y_start;

            let mut tile_img = RgbImage::new(tile_width, tile_height);

            for py in 0..tile_height {
                for px in 0..tile_width {
                    let global_x = x_start + px;
                    let global_y = y_start + py;

                    let x = (global_x as f32 / (width - 1) as f32) * 2.0 - 1.0;
                    let y = (global_y as f32 / (height - 1) as f32) * 2.0 - 1.0;
                    let Colour { r, g, b } = function(PixelCoordinates { x, y });

                    let pixel = Rgb([
                        ((r + 1.0) * 127.5).clamp(0.0, 255.0) as u8,
                        ((g + 1.0) * 127.5).clamp(0.0, 255.0) as u8,
                        ((b + 1.0) * 127.5).clamp(0.0, 255.0) as u8,
                    ]);
                    tile_img.put_pixel(px, py, pixel);
                }
            }

            (x_start, y_start, tile_img)
        })
        .collect();

    let mut final_img = RgbImage::new(width, height);
    for (x_start, y_start, tile) in partial_tiles {
        for (px, py, pixel) in tile.enumerate_pixels() {
            final_img.put_pixel(x_start + px, y_start + py, *pixel);
        }
    }

    final_img
}