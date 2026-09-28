pub mod pixel_buffer;

use anyhow::Result;
use crate::{
    ftz::disable_ftz,
    op::Op,
    vm::{EvaluationStack, compile},
};
use pixel_buffer::PixelBuffer;
use rayon::prelude::*;

pub fn render(channels: &[Vec<Op>; 3], width: u32, height: u32) -> Result<PixelBuffer> {
    let mut buf = PixelBuffer::new(width, height);
    if buf.data.is_empty() {
        return Ok(buf);
    }
    let w = width as usize;
    rayon::broadcast(|_| disable_ftz());
    disable_ftz();

    let xs = positions(width);
    let ys = positions(height);
    let programs = channels.each_ref().map(|ops| compile(ops, &xs, &ys));

    buf.data.par_chunks_mut(w * 3).enumerate().for_each_init(
        || [EvaluationStack::new(w), EvaluationStack::new(w), EvaluationStack::new(w)],
        |[red, green, blue], (row, out)| {
            let r = programs[0].run(red, row);
            let g = programs[1].run(green, row);
            let b = programs[2].run(blue, row);

            for (px, pixel) in out.chunks_exact_mut(3).enumerate() {
                pixel[0] = to_byte(r[px]);
                pixel[1] = to_byte(g[px]);
                pixel[2] = to_byte(b[px]);
            }
        },
    );

    Ok(buf)
}

pub fn pixel_position(pixel: usize, size: u32) -> f32 {
    (pixel as f32 / (size - 1) as f32) * 2.0 - 1.0
}

fn positions(size: u32) -> Vec<f32> {
    (0..size as usize).map(|pixel| pixel_position(pixel, size)).collect()
}

pub fn to_byte(value: f32) -> u8 {
    ((value + 1.0) * 127.5).clamp(0.0, 255.0) as u8
}
