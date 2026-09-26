use crate::{Channel, Machine, pixel_positions, render_bands};
use anyhow::Result;
use engine::{
    backend::Backend,
    ftz::disable_ftz,
    op::Op,
    render::{pixel_buffer::PixelBuffer, pixel_position},
};

pub struct MemoStackVm;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Label {
    Const,
    XOnly,
    YOnly,
    Xy,
}

fn combine(a: Label, b: Label) -> Label {
    match (a, b) {
        (Label::Const, other) | (other, Label::Const) => other,
        (a, b) if a == b => a,
        _ => Label::Xy,
    }
}

fn label(ops: &[Op]) -> (Vec<Label>, Vec<usize>) {
    let mut labels = vec![Label::Const; ops.len()];
    let mut sizes = vec![1; ops.len()];
    let mut stack: Vec<(Label, usize)> = Vec::new();
    for (i, &op) in ops.iter().enumerate().rev() {
        let mut here = match op {
            Op::X => Label::XOnly,
            Op::Y => Label::YOnly,
            _ => Label::Const,
        };
        let mut size = 1;
        for _ in 0..op.arity() {
            let (child, child_size) = stack.pop().unwrap();
            here = combine(here, child);
            size += child_size;
        }
        labels[i] = here;
        sizes[i] = size;
        stack.push((here, size));
    }
    (labels, sizes)
}

#[derive(Clone, Copy)]
enum Inst {
    Op(Op),
    Column(usize),
    Row(usize),
}

struct Program {
    main: Vec<Inst>,
    columns: Vec<Vec<f32>>,
    rows: Vec<Vec<f32>>,
}

fn prepare(ops: &[Op], xs: &[f32], height: u32) -> Program {
    let (labels, sizes) = label(ops);
    let mut program = Program { main: Vec::new(), columns: Vec::new(), rows: Vec::new() };
    let mut scalar = Machine::new(1);
    let mut wide = Machine::new(xs.len());

    let mut i = 0;
    while i < ops.len() {
        let subtree = &ops[i..i + sizes[i]];
        match labels[i] {
            Label::Xy => {
                program.main.push(Inst::Op(ops[i]));
                i += 1;
                continue;
            }
            Label::Const => {
                let value = scalar.run(subtree, &[0.0], 0.0)[0];
                program.main.push(Inst::Op(Op::Const(value)));
            }
            Label::XOnly => {
                program.columns.push(wide.run(subtree, xs, 0.0).to_vec());
                program.main.push(Inst::Column(program.columns.len() - 1));
            }
            Label::YOnly => {
                let mut values = Vec::with_capacity(height as usize);
                for row in 0..height as usize {
                    values.push(scalar.run(subtree, &[0.0], pixel_position(row, height))[0]);
                }
                program.rows.push(values);
                program.main.push(Inst::Row(program.rows.len() - 1));
            }
        }
        i += sizes[i];
    }
    program
}

impl Channel for Program {
    fn run_row<'m>(&self, machine: &'m mut Machine, row: usize, xs: &[f32], y: f32) -> &'m [f32] {
        machine.top = 0;
        for &inst in self.main.iter().rev() {
            match inst {
                Inst::Op(op) => machine.step(op, xs, y),
                Inst::Column(k) => machine.push().copy_from_slice(&self.columns[k]),
                Inst::Row(k) => machine.push().fill(self.rows[k][row]),
            }
        }
        &machine.stack[0]
    }
}

impl Backend for MemoStackVm {
    fn render(&self, channels: &[Vec<Op>; 3], width: u32, height: u32) -> Result<PixelBuffer> {
        let xs = pixel_positions(width);
        disable_ftz();
        let programs = channels.each_ref().map(|ops| prepare(ops, &xs, height));
        Ok(render_bands(&programs, &xs, width, height))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_follow_dependencies() {
        let ops = [Op::Add, Op::Sin, Op::X, Op::Y];
        let (labels, sizes) = label(&ops);
        assert_eq!(labels, [Label::Xy, Label::XOnly, Label::XOnly, Label::YOnly]);
        assert_eq!(sizes, [4, 2, 1, 1]);
    }

    #[test]
    fn constants_stay_constant() {
        let ops = [Op::Mult, Op::Const(0.5), Op::Cos, Op::Const(0.25)];
        let (labels, _) = label(&ops);
        assert_eq!(labels, [Label::Const; 4]);
    }
}
