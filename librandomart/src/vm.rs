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
    Cached(usize),
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

pub enum CacheEntry {
    XOnly(Vec<f32>),
    YOnly(Vec<f32>),
}

pub struct Cache {
    pub entries: Vec<CacheEntry>,
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
    Nothing,
    X,
    Y,
    XAndY,
}

fn combine(a: DependsOn, b: DependsOn) -> DependsOn {
    match (a, b) {
        (DependsOn::Nothing, other) | (other, DependsOn::Nothing) => other,
        (a, b) if a == b => a,
        _ => DependsOn::XAndY,
    }
}

struct Subtree {
    depends_on: DependsOn,
    start: usize,
    size: usize,
    placeholder: Option<usize>,
}

fn instruction(op: Op) -> Instruction {
    match op {
        Op::X | Op::Y => Instruction::Cached(0),
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
        cache: Cache { entries: vec![CacheEntry::XOnly(inputs.to_vec())] },
    };
    let mut stack = EvaluationStack::new(inputs.len());
    program.run(&mut stack, 0).to_vec()
}

fn cut(ops: &[Op], subtree: &Subtree, xs: &[f32], ys: &[f32], entries: &mut Vec<CacheEntry>) -> Instruction {
    let subtree_ops = &ops[subtree.start..subtree.start + subtree.size];
    match subtree.depends_on {
        DependsOn::Nothing => Instruction::Const(evaluate(subtree_ops, &[0.0])[0]),
        DependsOn::X => {
            entries.push(CacheEntry::XOnly(evaluate(subtree_ops, xs)));
            Instruction::Cached(entries.len() - 1)
        }
        DependsOn::Y => {
            entries.push(CacheEntry::YOnly(evaluate(subtree_ops, ys)));
            Instruction::Cached(entries.len() - 1)
        }
        DependsOn::XAndY => unreachable!(),
    }
}

impl Program {
    pub fn new(ops: &[Op], width: u32, height: u32) -> Self {
        let xs = positions(width);
        let ys = positions(height);
        let mut entries = Vec::new();
        let mut output: Vec<Option<Instruction>> = Vec::new();
        let mut subtrees: Vec<Subtree> = Vec::new();

        for (i, &op) in ops.iter().enumerate().rev() {
            let children: Vec<Subtree> = (0..op.arity()).map(|_| subtrees.pop().unwrap()).collect();
            let mut depends_on = match op {
                Op::X => DependsOn::X,
                Op::Y => DependsOn::Y,
                _ => DependsOn::Nothing,
            };
            let mut size = 1;
            for child in &children {
                depends_on = combine(depends_on, child.depends_on);
                size += child.size;
            }

            if depends_on == DependsOn::XAndY {
                for child in &children {
                    if let Some(placeholder) = child.placeholder {
                        output[placeholder] = Some(cut(ops, child, &xs, &ys, &mut entries));
                    }
                }
                output.push(Some(instruction(op)));
                subtrees.push(Subtree { depends_on, start: i, size, placeholder: None });
            } else {
                output.truncate(output.len() - children.len());
                output.push(None);
                subtrees.push(Subtree { depends_on, start: i, size, placeholder: Some(output.len() - 1) });
            }
        }

        let root = subtrees.pop().unwrap();
        if let Some(placeholder) = root.placeholder {
            output[placeholder] = Some(cut(ops, &root, &xs, &ys, &mut entries));
        }

        let instructions = output.into_iter().map(Option::unwrap).collect();
        Self { instructions, cache: Cache { entries } }
    }

    pub fn run<'s>(&self, stack: &'s mut EvaluationStack, row: usize) -> &'s [f32] {
        stack.stack_ptr = 0;
        for &instruction in self.instructions.iter() {
            match instruction {
                Instruction::Cached(k) => match &self.cache.entries[k] {
                    CacheEntry::XOnly(values) => stack.push().copy_from_slice(values),
                    CacheEntry::YOnly(values) => stack.push().fill(values[row]),
                },
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
