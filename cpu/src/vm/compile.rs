use super::{Cache, EvaluationStack, Instruction, Program};
use randomart::{
    op::Op,
    subtree::{DependsOn, Subtree, depends_on, subtrees},
};

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

struct SubtreeEvaluator {
    program: Program,
    stack: EvaluationStack,
}

impl SubtreeEvaluator {
    fn new(inputs: &[f32]) -> Self {
        Self {
            program: Program {
                instructions: Vec::new(),
                cache: Cache { x_rows: vec![inputs.to_vec()], y_values: Vec::new() },
            },
            stack: EvaluationStack::new(inputs.len()),
        }
    }

    fn evaluate(&mut self, ops: &[Op]) -> &[f32] {
        self.program.instructions.clear();
        self.program.instructions.extend(ops.iter().rev().map(|&op| instruction(op)));
        self.program.run(&mut self.stack, 0)
    }
}

fn cache_subtree(
    subtree: &[Op],
    depends_on: DependsOn,
    constant: &mut SubtreeEvaluator,
    x: &mut SubtreeEvaluator,
    y: &mut SubtreeEvaluator,
    cache: &mut Cache,
) -> Instruction {
    match depends_on {
        DependsOn::Constant => Instruction::Const(constant.evaluate(subtree)[0]),
        DependsOn::XOnly => {
            cache.x_rows.push(x.evaluate(subtree).to_vec());
            Instruction::XOnlySubtree(cache.x_rows.len() - 1)
        }
        DependsOn::YOnly => {
            cache.y_values.push(y.evaluate(subtree).to_vec());
            Instruction::YOnlySubtree(cache.y_values.len() - 1)
        }
        DependsOn::XY => unreachable!(),
    }
}

fn precompute(ops: &[Op], dependencies: &[(DependsOn, usize)], xs: &[f32], ys: &[f32]) -> Program {
    let mut cache = Cache { x_rows: Vec::new(), y_values: Vec::new() };
    let mut constant = SubtreeEvaluator::new(&[0.0]);
    let mut x = SubtreeEvaluator::new(xs);
    let mut y = SubtreeEvaluator::new(ys);
    let mut instructions = Vec::new();
    let mut cached_until = 0;
    for subtree in subtrees(ops) {
        if let Subtree::Start { index } = subtree {
            if index < cached_until {
                continue;
            }
            let (depends_on, size) = dependencies[index];
            if depends_on == DependsOn::XY {
                instructions.push(instruction(ops[index]));
            } else {
                instructions.push(cache_subtree(&ops[index..index + size], depends_on, &mut constant, &mut x, &mut y, &mut cache));
                cached_until = index + size;
            }
        }
    }
    instructions.reverse();
    Program { instructions, cache }
}

pub fn compile(ops: &[Op], xs: &[f32], ys: &[f32]) -> Program {
    let dependencies = depends_on(ops);
    precompute(ops, &dependencies, xs, ys)
}
