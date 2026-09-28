use super::{Cache, EvaluationStack, Instruction, Program};
use crate::op::Op;

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

fn label(ops: &[Op]) -> Vec<(DependsOn, usize)> {
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

enum Subtree<'a> {
    NonReducible(Op),
    Reducible(DependsOn, &'a [Op]),
}

fn reduce_subtrees<'a>(ops: &'a [Op], labels: &[(DependsOn, usize)]) -> Vec<Subtree<'a>> {
    let mut subtrees = Vec::new();
    let mut i = 0;
    while i < ops.len() {
        let (depends_on, size) = labels[i];
        if depends_on == DependsOn::XY {
            subtrees.push(Subtree::NonReducible(ops[i]));
            i += 1;
        } else {
            subtrees.push(Subtree::Reducible(depends_on, &ops[i..i + size]));
            i += size;
        }
    }
    subtrees
}

fn precompute(subtrees: &[Subtree], xs: &[f32], ys: &[f32]) -> Program {
    let mut cache = Cache { x_rows: Vec::new(), y_values: Vec::new() };
    let instructions = subtrees
        .iter()
        .rev()
        .map(|subtree| match *subtree {
            Subtree::NonReducible(op) => instruction(op),
            Subtree::Reducible(depends_on, ops) => cache_subtree(ops, depends_on, xs, ys, &mut cache),
        })
        .collect();
    Program { instructions, cache }
}

pub fn compile(ops: &[Op], xs: &[f32], ys: &[f32]) -> Program {
    let labels = label(ops);
    let subtrees = reduce_subtrees(ops, &labels);
    precompute(&subtrees, xs, ys)
}
