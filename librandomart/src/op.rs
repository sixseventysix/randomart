#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    X,
    Y,
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

impl Op {
    pub fn arity(self) -> usize {
        match self {
            Op::X | Op::Y | Op::Const(_) => 0,
            Op::Sqrt | Op::Sin | Op::Cos | Op::Exp => 1,
            Op::Add | Op::Mult | Op::Div => 2,
            Op::Mix => 4,
        }
    }
}

#[inline]
pub fn sqrt(a: f32) -> f32 {
    a.sqrt().max(0.0)
}

#[inline]
pub fn sin(a: f32) -> f32 {
    math::sinf(a)
}

#[inline]
pub fn cos(a: f32) -> f32 {
    math::cosf(a)
}

#[inline]
pub fn exp(a: f32) -> f32 {
    math::expf(a)
}

#[inline]
pub fn add(a: f32, b: f32) -> f32 {
    (a + b) / 2.0
}

#[inline]
pub fn mult(a: f32, b: f32) -> f32 {
    a * b
}

#[inline]
pub fn div(a: f32, b: f32) -> f32 {
    if b.abs() > 1e-6 { a / b } else { 0.0 }
}

#[inline]
pub fn mix(a: f32, b: f32, c: f32, d: f32) -> f32 {
    (a * c + b * d) / (a + b + 1e-6)
}

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

pub fn check(ops: &[Op]) -> anyhow::Result<()> {
    let mut open = 1;
    for subtree in subtrees(ops) {
        if let Subtree::Start { index } = subtree {
            if open == 0 {
                anyhow::bail!("{:?} at index {index} comes after the expression is already complete", ops[index]);
            }
            open = open - 1 + ops[index].arity();
        }
    }
    if open != 0 {
        anyhow::bail!("the expression is incomplete: {open} inputs are missing");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{check, subtrees, Subtree, Op::*};

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

    #[test]
    fn accepts_a_complete_expression() {
        assert!(check(&[Add, Sin, X, Mult, X, Const(0.5)]).is_ok());
    }

    #[test]
    fn rejects_missing_inputs() {
        assert!(check(&[Add]).is_err());
        assert!(check(&[Add, X]).is_err());
        assert!(check(&[]).is_err());
    }

    #[test]
    fn rejects_extra_ops() {
        assert!(check(&[X, Y]).is_err());
    }
}
