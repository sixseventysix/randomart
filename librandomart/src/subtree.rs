use crate::op::Op;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Subtree {
    Start { index: usize },
    End { index: usize, size: usize },
}

pub fn subtrees(ops: &[Op]) -> impl Iterator<Item = Subtree> + '_ {
    let mut unfinished: Vec<(usize, usize)> = Vec::new();
    let mut next = 0;
    std::iter::from_fn(move || {
        if let Some(&(index, 0)) = unfinished.last() {
            unfinished.pop();
            if let Some((_, remaining)) = unfinished.last_mut() {
                *remaining -= 1;
            }
            return Some(Subtree::End { index, size: next - index });
        }
        let op = *ops.get(next)?;
        unfinished.push((next, op.arity()));
        next += 1;
        Some(Subtree::Start { index: next - 1 })
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DependsOn {
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

pub fn depends_on(ops: &[Op]) -> Vec<(DependsOn, usize)> {
    let mut dependencies = vec![(DependsOn::Constant, 0); ops.len()];
    let mut stack: Vec<DependsOn> = Vec::new();
    for subtree in subtrees(ops) {
        if let Subtree::End { index, size } = subtree {
            let op = ops[index];
            let leaf = match op {
                Op::X => DependsOn::XOnly,
                Op::Y => DependsOn::YOnly,
                _ => DependsOn::Constant,
            };
            let depends_on = stack.drain(stack.len() - op.arity()..).fold(leaf, combine);
            stack.push(depends_on);
            dependencies[index] = (depends_on, size);
        }
    }
    dependencies
}

#[cfg(test)]
mod tests {
    use super::{subtrees, Subtree};
    use crate::op::Op::*;

    #[test]
    fn subtrees_start_and_end_in_order() {
        assert_eq!(
            subtrees(&[Mult, X, Sin, Y]).collect::<Vec<_>>(),
            [
                Subtree::Start { index: 0 },
                Subtree::Start { index: 1 },
                Subtree::End { index: 1, size: 1 },
                Subtree::Start { index: 2 },
                Subtree::Start { index: 3 },
                Subtree::End { index: 3, size: 1 },
                Subtree::End { index: 2, size: 2 },
                Subtree::End { index: 0, size: 4 },
            ]
        );
    }
}
