#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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

pub fn check(ops: &[Op]) -> anyhow::Result<()> {
    let mut open = 1;
    for (index, op) in ops.iter().enumerate() {
        if open == 0 {
            anyhow::bail!("{op:?} at index {index} comes after the expression is already complete");
        }
        open = open - 1 + op.arity();
    }
    if open != 0 {
        anyhow::bail!("the expression is incomplete: {open} inputs are missing");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{check, Op::*};

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
