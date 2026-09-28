use anyhow::Result;
use crate::{
    ftz::disable_ftz,
    math,
    op::Op,
    render::{pixel_buffer::PixelBuffer, pixel_position, to_byte},
};
use rayon::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
    XOnlySubtree(usize),
    YOnlySubtree(usize),
    Const(f32),
    Sqrt,
    Sin,
    Cos,
    Exp,
    Add,
    Mult,
    Div,
    Mix,
}

pub struct Cache {
    pub x_rows: Vec<Vec<f32>>,
    pub y_values: Vec<Vec<f32>>,
}

pub type SubtreeEvaluations = Vec<f32>;

pub struct EvaluationStack {
    evaluations: Vec<SubtreeEvaluations>,
    stack_ptr: usize,
    width: usize,
}

impl EvaluationStack {
    pub fn new(width: usize) -> Self {
        Self { evaluations: Vec::new(), stack_ptr: 0, width }
    }

    fn push(&mut self) -> &mut [f32] {
        if self.stack_ptr == self.evaluations.len() {
            self.evaluations.push(vec![0.0; self.width]);
        }
        self.stack_ptr += 1;
        &mut self.evaluations[self.stack_ptr - 1]
    }

    fn unary(&mut self, f: impl Fn(f32) -> f32) {
        for v in self.evaluations[self.stack_ptr - 1].iter_mut() {
            *v = f(*v);
        }
    }

    fn unary_row(&mut self, f: fn(&mut [f32])) {
        f(&mut self.evaluations[self.stack_ptr - 1]);
    }

    fn binary(&mut self, f: impl Fn(f32, f32) -> f32) {
        let (lower, upper) = self.evaluations.split_at_mut(self.stack_ptr - 1);
        let a = &upper[0];
        let b = &mut lower[self.stack_ptr - 2];
        for (b, &a) in b.iter_mut().zip(a) {
            *b = f(a, *b);
        }
        self.stack_ptr -= 1;
    }

    fn mix(&mut self) {
        let (lower, upper) = self.evaluations.split_at_mut(self.stack_ptr - 3);
        let a = &upper[2];
        let b = &upper[1];
        let c = &upper[0];
        let d = &mut lower[self.stack_ptr - 4];
        for (((d, &a), &b), &c) in d.iter_mut().zip(a).zip(b).zip(c) {
            *d = (a * c + b * *d) / (a + b + 1e-6);
        }
        self.stack_ptr -= 3;
    }
}

pub struct Program {
    instructions: Vec<Instruction>,
    cache: Cache,
}

fn positions(size: u32) -> Vec<f32> {
    (0..size as usize).map(|pixel| pixel_position(pixel, size)).collect()
}

#[derive(Clone, Copy, PartialEq)]
enum DependsOn {
    Constant,
    XOnly,
    YOnly,
    XY,
}

fn combine(a: DependsOn, b: DependsOn) -> DependsOn {
    match (a, b) {
        (DependsOn::Constant, other) | (other, DependsOn::Constant) => other,
        (a, b) if a == b => a,
        _ => DependsOn::XY,
    }
}

fn depends_on(ops: &[Op]) -> Vec<(DependsOn, usize)> {
    let mut stack: Vec<(DependsOn, usize)> = Vec::new();
    let mut nodes: Vec<(DependsOn, usize)> = ops
        .iter()
        .rev()
        .map(|&op| {
            let leaf = match op {
                Op::X => DependsOn::XOnly,
                Op::Y => DependsOn::YOnly,
                _ => DependsOn::Constant,
            };
            let node = stack
                .drain(stack.len() - op.arity()..)
                .fold((leaf, 1), |(depends_on, size), (child, child_size)| (combine(depends_on, child), size + child_size));
            stack.push(node);
            node
        })
        .collect();
    nodes.reverse();
    nodes
}

fn instruction(op: Op) -> Instruction {
    match op {
        Op::X | Op::Y => Instruction::XOnlySubtree(0),
        Op::Const(v) => Instruction::Const(v),
        Op::Sqrt => Instruction::Sqrt,
        Op::Sin => Instruction::Sin,
        Op::Cos => Instruction::Cos,
        Op::Exp => Instruction::Exp,
        Op::Add => Instruction::Add,
        Op::Mult => Instruction::Mult,
        Op::Div => Instruction::Div,
        Op::Mix => Instruction::Mix,
    }
}

fn evaluate(ops: &[Op], inputs: &[f32]) -> Vec<f32> {
    let program = Program {
        instructions: ops.iter().rev().map(|&op| instruction(op)).collect(),
        cache: Cache { x_rows: vec![inputs.to_vec()], y_values: Vec::new() },
    };
    let mut stack = EvaluationStack::new(inputs.len());
    program.run(&mut stack, 0).to_vec()
}

fn cache_subtree(subtree: &[Op], depends_on: DependsOn, xs: &[f32], ys: &[f32], cache: &mut Cache) -> Instruction {
    match depends_on {
        DependsOn::Constant => Instruction::Const(evaluate(subtree, &[0.0])[0]),
        DependsOn::XOnly => {
            cache.x_rows.push(evaluate(subtree, xs));
            Instruction::XOnlySubtree(cache.x_rows.len() - 1)
        }
        DependsOn::YOnly => {
            cache.y_values.push(evaluate(subtree, ys));
            Instruction::YOnlySubtree(cache.y_values.len() - 1)
        }
        DependsOn::XY => unreachable!(),
    }
}

impl Program {
    pub fn new(ops: &[Op], width: u32, height: u32) -> Self {
        let xs = positions(width);
        let ys = positions(height);
        let nodes = depends_on(ops);
        let mut cache = Cache { x_rows: Vec::new(), y_values: Vec::new() };
        let mut parents: Vec<(DependsOn, usize)> = Vec::new();
        let mut instructions: Vec<Instruction> = ops
            .iter()
            .zip(&nodes)
            .enumerate()
            .filter_map(|(i, (&op, &(depends_on, size)))| {
                let parent = parents.last().map(|&(parent, _)| parent);
                if let Some(waiting) = parents.last_mut() {
                    waiting.1 -= 1;
                    if waiting.1 == 0 {
                        parents.pop();
                    }
                }
                if op.arity() > 0 {
                    parents.push((depends_on, op.arity()));
                }
                match (depends_on, parent) {
                    (DependsOn::XY, _) => Some(instruction(op)),
                    (_, Some(DependsOn::XY) | None) => Some(cache_subtree(&ops[i..i + size], depends_on, &xs, &ys, &mut cache)),
                    _ => None,
                }
            })
            .collect();
        instructions.reverse();
        Self { instructions, cache }
    }

    pub fn run<'s>(&self, stack: &'s mut EvaluationStack, row: usize) -> &'s [f32] {
        stack.stack_ptr = 0;
        for &instruction in self.instructions.iter() {
            match instruction {
                Instruction::XOnlySubtree(k) => stack.push().copy_from_slice(&self.cache.x_rows[k]),
                Instruction::YOnlySubtree(k) => stack.push().fill(self.cache.y_values[k][row]),
                Instruction::Const(v) => stack.push().fill(v),
                Instruction::Sin => stack.unary_row(math::sinf_row),
                Instruction::Cos => stack.unary_row(math::cosf_row),
                Instruction::Exp => stack.unary_row(math::expf_row),
                Instruction::Sqrt => stack.unary(|a| math::sqrtf(a).max(0.0)),
                Instruction::Add => stack.binary(|a, b| (a + b) / 2.0),
                Instruction::Mult => stack.binary(|a, b| a * b),
                Instruction::Div => stack.binary(|a, b| if b.abs() > 1e-6 { a / b } else { 0.0 }),
                Instruction::Mix => stack.mix(),
            }
        }
        &stack.evaluations[0]
    }
}

pub struct Vm;

impl Vm {
    pub fn render(&self, channels: &[Vec<Op>; 3], width: u32, height: u32) -> Result<PixelBuffer> {
        let mut buf = PixelBuffer::new(width, height);
        if buf.data.is_empty() {
            return Ok(buf);
        }
        let w = width as usize;
        rayon::broadcast(|_| disable_ftz());
        disable_ftz();

        let programs = channels.each_ref().map(|ops| Program::new(ops, width, height));

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
}
