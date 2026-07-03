use randomart_core::node::Node;
use randomart_core::math;

pub trait ClosureNode: Fn(f32, f32) -> f32 + Send + Sync {}
impl<T: Fn(f32, f32) -> f32 + Send + Sync> ClosureNode for T {}

pub fn compile_node(node: &Node) -> Box<dyn ClosureNode> {
    match node {
        Node::X => Box::new(|x, _| x),
        Node::Y => Box::new(|_, y| y),
        Node::Number(v) => {
            let val = *v;
            Box::new(move |_, _| val)
        }

        Node::Add(a, b) => {
            let fa = compile_node(a);
            let fb = compile_node(b);
            Box::new(move |x, y| (fa(x, y) + fb(x, y)) / 2.0)
        }
        Node::Mult(a, b) => {
            let fa = compile_node(a);
            let fb = compile_node(b);
            Box::new(move |x, y| fa(x, y) * fb(x, y))
        }
        Node::Div(a, b) => {
            let fa = compile_node(a);
            let fb = compile_node(b);
            Box::new(move |x, y| {
                let denom = fb(x, y);
                if denom.abs() > 1e-6 { fa(x, y) / denom } else { 0.0 }
            })
        }
        Node::Sqrt(inner) => {
            let f = compile_node(inner);
            Box::new(move |x, y| math::sqrtf(f(x, y)).max(0.0))
        }
        Node::Sin(inner) => {
            let f = compile_node(inner);
            Box::new(move |x, y| math::sinf(f(x, y)))
        }
        Node::Cos(inner) => {
            let f = compile_node(inner);
            Box::new(move |x, y| math::cosf(f(x, y)))
        }
        Node::Exp(inner) => {
            let f = compile_node(inner);
            Box::new(move |x, y| math::expf(f(x, y)))
        }
        Node::MixUnbounded(a, b, c, d) => {
            let fa = compile_node(a);
            let fb = compile_node(b);
            let fc = compile_node(c);
            let fd = compile_node(d);
            Box::new(move |x, y| {
                let a = fa(x, y);
                let b = fb(x, y);
                let c = fc(x, y);
                let d = fd(x, y);
                (a * c + b * d) / (a + b + 1e-6)
            })
        }

        Node::Random => panic!("Node::Random should be resolved before compilation"),
        Node::Triple(_, _, _) => panic!("compile_node() is for scalar nodes, not Triple"),
        node => unimplemented!("compile_node: missing match arm for {:?}", node),
    }
}
