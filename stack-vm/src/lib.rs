use anyhow::Result;
use engine::{
    backend::Backend,
    ftz::disable_ftz,
    math,
    op::Op,
    render::{pixel_buffer::PixelBuffer, pixel_position, to_byte},
};
use rayon::prelude::*;

const ROWS_PER_BAND: usize = 4;

pub struct StackVm;

struct Machine {
    stack: Vec<Vec<f32>>,
    top: usize,
    width: usize,
}

impl Machine {
    fn new(width: usize) -> Self {
        Self { stack: Vec::new(), top: 0, width }
    }

    fn push(&mut self) -> &mut [f32] {
        if self.top == self.stack.len() {
            self.stack.push(vec![0.0; self.width]);
        }
        self.top += 1;
        &mut self.stack[self.top - 1]
    }

    fn unary(&mut self, f: impl Fn(f32) -> f32) {
        for v in self.stack[self.top - 1].iter_mut() {
            *v = f(*v);
        }
    }

    fn binary(&mut self, f: impl Fn(f32, f32) -> f32) {
        let (lower, upper) = self.stack.split_at_mut(self.top - 1);
        let a = &upper[0];
        let b = &mut lower[self.top - 2];
        for (b, &a) in b.iter_mut().zip(a) {
            *b = f(a, *b);
        }
        self.top -= 1;
    }

    fn mix(&mut self) {
        let (lower, upper) = self.stack.split_at_mut(self.top - 3);
        let a = &upper[2];
        let b = &upper[1];
        let c = &upper[0];
        let d = &mut lower[self.top - 4];
        for i in 0..self.width {
            d[i] = (a[i] * c[i] + b[i] * d[i]) / (a[i] + b[i] + 1e-6);
        }
        self.top -= 3;
    }

    fn run(&mut self, ops: &[Op], xs: &[f32], y: f32) -> &[f32] {
        self.top = 0;
        for &op in ops.iter().rev() {
            match op {
                Op::X => self.push().copy_from_slice(xs),
                Op::Y => self.push().fill(y),
                Op::Const(v) => self.push().fill(v),
                Op::Sin => self.unary(math::sinf),
                Op::Cos => self.unary(math::cosf),
                Op::Exp => self.unary(math::expf),
                Op::Sqrt => self.unary(|a| math::sqrtf(a).max(0.0)),
                Op::Add => self.binary(|a, b| (a + b) / 2.0),
                Op::Mult => self.binary(|a, b| a * b),
                Op::Div => self.binary(|a, b| if b.abs() > 1e-6 { a / b } else { 0.0 }),
                Op::Mix => self.mix(),
            }
        }
        &self.stack[0]
    }
}

impl Backend for StackVm {
    fn render(&self, channels: &[Vec<Op>; 3], width: u32, height: u32) -> Result<PixelBuffer> {
        let mut buf = PixelBuffer::new(width, height);
        if buf.data.is_empty() {
            return Ok(buf);
        }
        let row_bytes = width as usize * 3;
        let xs: Vec<f32> = (0..width as usize).map(|px| pixel_position(px, width)).collect();

        rayon::broadcast(|_| disable_ftz());

        buf.data
            .par_chunks_mut(row_bytes * ROWS_PER_BAND)
            .enumerate()
            .for_each(|(band, out)| {
                let mut machines = [
                    Machine::new(width as usize),
                    Machine::new(width as usize),
                    Machine::new(width as usize),
                ];
                let [red, green, blue] = &mut machines;

                for (i, row) in out.chunks_mut(row_bytes).enumerate() {
                    let y = pixel_position(band * ROWS_PER_BAND + i, height);
                    let r = red.run(&channels[0], &xs, y);
                    let g = green.run(&channels[1], &xs, y);
                    let b = blue.run(&channels[2], &xs, y);

                    for (px, pixel) in row.chunks_exact_mut(3).enumerate() {
                        pixel[0] = to_byte(r[px]);
                        pixel[1] = to_byte(g[px]);
                        pixel[2] = to_byte(b[px]);
                    }
                }
            });

        Ok(buf)
    }
}
