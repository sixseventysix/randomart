mod analysis;
mod run;

pub use analysis::compile;
pub use run::EvaluationStack;

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

pub struct Program {
    instructions: Vec<Instruction>,
    cache: Cache,
}
