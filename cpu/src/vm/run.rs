use super::{Instruction, Program};
use randomart::op;

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
            *d = op::mix(a, b, c, *d);
        }
        self.stack_ptr -= 3;
    }
}

impl Program {
    pub fn run<'s>(&self, stack: &'s mut EvaluationStack, row: usize) -> &'s [f32] {
        stack.stack_ptr = 0;
        for &instruction in self.instructions.iter() {
            match instruction {
                Instruction::XOnlySubtree(k) => stack.push().copy_from_slice(&self.cache.x_rows[k]),
                Instruction::YOnlySubtree(k) => stack.push().fill(self.cache.y_values[k][row]),
                Instruction::Const(v) => stack.push().fill(v),
                Instruction::Sin => stack.unary(op::sin),
                Instruction::Cos => stack.unary(op::cos),
                Instruction::Exp => stack.unary(op::exp),
                Instruction::Sqrt => stack.unary(op::sqrt),
                Instruction::Add => stack.binary(op::add),
                Instruction::Mult => stack.binary(op::mult),
                Instruction::Div => stack.binary(op::div),
                Instruction::Mix => stack.mix(),
            }
        }
        &stack.evaluations[0]
    }
}
