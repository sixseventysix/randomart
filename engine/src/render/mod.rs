pub mod pixel_buffer;

use crate::ftz::disable_ftz;
use pixel_buffer::PixelBuffer;
use rayon::prelude::*;

const ROWS_PER_BAND: usize = 4;

pub trait ColourChannel: Sync + Fn(f32, f32) -> f32 {}
impl<T: Sync + Fn(f32, f32) -> f32> ColourChannel for T {}

pub fn pixel_position(pixel: usize, size: u32) -> f32 {
    (pixel as f32 / (size - 1) as f32) * 2.0 - 1.0
}

pub fn to_byte(value: f32) -> u8 {
    ((value + 1.0) * 127.5).clamp(0.0, 255.0) as u8
}

fn fill_row(channel: &impl ColourChannel, y: f32, width: u32, row: &mut [f32]) {
    for (px, value) in row.iter_mut().enumerate() {
        *value = channel(pixel_position(px, width), y);
    }
}

/// Render `width x height` pixels in parallel row bands by evaluating each
/// channel at every pixel's `[-1, 1]` coordinate. Disables FTZ/DAZ on every worker
/// thread so subnormal floats are handled IEEE-correctly, keeping CPU backends bit-exact.
pub fn render_channels(
    r: impl ColourChannel,
    g: impl ColourChannel,
    b: impl ColourChannel,
    width: u32,
    height: u32,
) -> PixelBuffer {
    let mut buf = PixelBuffer::new(width, height);
    if buf.data.is_empty() {
        return buf;
    }
    let row_bytes = width as usize * 3;

    rayon::broadcast(|_| disable_ftz());

    buf.data
        .par_chunks_mut(row_bytes * ROWS_PER_BAND)
        .enumerate()
        .for_each(|(band, out)| {
            let mut red = vec![0.0; width as usize];
            let mut green = vec![0.0; width as usize];
            let mut blue = vec![0.0; width as usize];

            for (i, row) in out.chunks_mut(row_bytes).enumerate() {
                let y = pixel_position(band * ROWS_PER_BAND + i, height);
                fill_row(&r, y, width, &mut red);
                fill_row(&g, y, width, &mut green);
                fill_row(&b, y, width, &mut blue);

                for (px, pixel) in row.chunks_exact_mut(3).enumerate() {
                    pixel[0] = to_byte(red[px]);
                    pixel[1] = to_byte(green[px]);
                    pixel[2] = to_byte(blue[px]);
                }
            }
        });

    buf
}
