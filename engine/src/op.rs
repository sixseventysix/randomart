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
