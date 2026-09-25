use engine::math;
use engine::op::Op;

pub trait ClosureNode: Fn(f32, f32) -> f32 + Send + Sync {}
impl<T: Fn(f32, f32) -> f32 + Send + Sync> ClosureNode for T {}

pub fn compile(ops: &[Op]) -> Box<dyn ClosureNode> {
    let mut stack: Vec<Box<dyn ClosureNode>> = Vec::new();
    for &op in ops.iter().rev() {
        let f: Box<dyn ClosureNode> = match op {
            Op::X => Box::new(|x, _| x),
            Op::Y => Box::new(|_, y| y),
            Op::Const(v) => Box::new(move |_, _| v),
            Op::Sin => {
                let a = stack.pop().unwrap();
                Box::new(move |x, y| math::sinf(a(x, y)))
            }
            Op::Cos => {
                let a = stack.pop().unwrap();
                Box::new(move |x, y| math::cosf(a(x, y)))
            }
            Op::Exp => {
                let a = stack.pop().unwrap();
                Box::new(move |x, y| math::expf(a(x, y)))
            }
            Op::Sqrt => {
                let a = stack.pop().unwrap();
                Box::new(move |x, y| math::sqrtf(a(x, y)).max(0.0))
            }
            Op::Add => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                Box::new(move |x, y| (a(x, y) + b(x, y)) / 2.0)
            }
            Op::Mult => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                Box::new(move |x, y| a(x, y) * b(x, y))
            }
            Op::Div => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                Box::new(move |x, y| {
                    let denom = b(x, y);
                    if denom.abs() > 1e-6 { a(x, y) / denom } else { 0.0 }
                })
            }
            Op::Mix => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let c = stack.pop().unwrap();
                let d = stack.pop().unwrap();
                Box::new(move |x, y| {
                    let a = a(x, y);
                    let b = b(x, y);
                    let c = c(x, y);
                    let d = d(x, y);
                    (a * c + b * d) / (a + b + 1e-6)
                })
            }
        };
        stack.push(f);
    }
    stack.pop().unwrap()
}
