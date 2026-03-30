use randomart_core::pixel_buffer::PixelBuffer;
use randomart_core::node::Node;
use randomart_core::math;

pub struct PixelCoordinates {
    pub x: f32,
    pub y: f32,
}

pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub fn render_pixels<F>(function: F, width: u32, height: u32) -> PixelBuffer
where
    F: Fn(PixelCoordinates) -> Colour,
{
    let mut buf = PixelBuffer::new(width, height);

    for py in 0..height {
        for px in 0..width {
            let x = (px as f32 / (width - 1) as f32) * 2.0 - 1.0;
            let y = (py as f32 / (height - 1) as f32) * 2.0 - 1.0;
            let colour = function(PixelCoordinates { x, y });
            let r = ((colour.r + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
            let g = ((colour.g + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
            let b = ((colour.b + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
            buf.put_pixel(px, py, r, g, b);
        }
    }

    buf
}

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
