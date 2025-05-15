use crate::node::Node;
use crate::utils::Colour;

pub trait ClosureNode: Fn(f32, f32) -> f32 + Send + Sync {}
impl<T: Fn(f32, f32) -> f32 + Send + Sync> ClosureNode for T {}

pub struct ClosureTree {
    pub r: Box<dyn ClosureNode>,
    pub g: Box<dyn ClosureNode>,
    pub b: Box<dyn ClosureNode>,
}

impl ClosureTree {
    pub fn from_node(node: &Node) -> Self {
        match node {
            Node::Triple(r, g, b) => Self {
                r: r.to_closure_tree(),
                g: g.to_closure_tree(),
                b: b.to_closure_tree(),
            },
            _ => panic!("Expected Node::Triple at top level"),
        }
    }

    pub fn eval_rgb(&self, x: f32, y: f32) -> Colour {
        Colour {
            r: (self.r)(x, y),
            g: (self.g)(x, y),
            b: (self.b)(x, y),
        }
    }
}