pub mod utils;
use utils::{Colour, LinearCongruentialGenerator};

#[derive(Clone, Debug)]
pub enum Node {
    X,                       // Represents the variable `x`
    Y,                       // Represents the variable `y`
    Random,                  // A random number
    Rule(usize),             // A reference to a grammar rule by index
    Number(f32),             // A constant number
    Boolean(bool),           // A boolean value
    Sqrt(Box<Node>),         // Square root operation (unary)
    Sin(Box<Node>),
    Cos(Box<Node>),
    Exp(Box<Node>),
    Add(Box<Node>, Box<Node>), // Addition (binary)
    Mult(Box<Node>, Box<Node>), // Multiplication (binary)
    Div(Box<Node>, Box<Node>),
    Mod(Box<Node>, Box<Node>),  // Modulus operation (binary)
    Gt(Box<Node>, Box<Node>),   // Greater-than comparison (binary)
    Triple(Box<Node>, Box<Node>, Box<Node>), // A triple node (e.g., RGB values)
    If {
        cond: Box<Node>,     // Condition for the `if`
        then: Box<Node>,     // `then` branch
        elze: Box<Node>,     // `else` branch
    },
    Mix(Box<Node>, Box<Node>, Box<Node>, Box<Node>)
}

impl Node {
    fn clone_with_operands(&self, lhs: Box<Node>, rhs: Box<Node>) -> Self {
        match self {
            Node::Add(_, _) => Node::Add(lhs, rhs),
            Node::Mult(_, _) => Node::Mult(lhs, rhs),
            Node::Mod(_, _) => Node::Mod(lhs, rhs),
            Node::Gt(_, _) => Node::Gt(lhs, rhs),
            _ => panic!("Invalid operation: clone_with_operands can only be called on binary operation nodes"),
        }
    }


    fn eval(&self, x: f32, y: f32) -> Option<f32> {
        match self {
            Node::X => Some(x),
            Node::Y => Some(y),
            Node::Number(value) => Some(*value),
            Node::Random => None,
            Node::Add(lhs, rhs) => {
                let lhs_val = lhs.eval(x, y)?;
                let rhs_val = rhs.eval(x, y)?;
                Some((lhs_val + rhs_val)/2.0)
            }
            Node::Mult(lhs, rhs) => {
                let lhs_val = lhs.eval(x, y)?;
                let rhs_val = rhs.eval(x, y)?;
                Some(lhs_val * rhs_val)
            }
            Node::Sin(inner) => {
                let val = inner.eval(x, y)?;
                Some(val.sin())
            }
            Node::Cos(inner) => {
                let val = inner.eval(x, y)?;
                Some(val.cos())
            }
            Node::Exp(inner) => {
                let val = inner.eval(x, y)?;
                Some(val.exp())
            }
            Node::Sqrt(inner) => {
                let val = inner.eval(x, y)?;
                Some(val.sqrt().max(0.0)) // Ensure non-negative output
            }
            Node::Div(lhs, rhs) => {
                let lhs_val = lhs.eval(x, y)?;
                let rhs_val = rhs.eval(x, y)?;
                if rhs_val.abs() > 1e-6 { // Prevent division by zero
                    Some(lhs_val / rhs_val)
                } else {
                    None
                }
            }
            Node::Mix(a, b, c, d) => {
                let a_val = a.eval(x, y)?;
                let b_val = b.eval(x, y)?;
                let c_val = c.eval(x, y)?;
                let d_val = d.eval(x, y)?;
                Some((a_val * c_val + b_val * d_val) / (a_val + b_val + 1e-6))
            }
            Node::Triple(_first, _second, _third) => {
                // Triple nodes should not directly evaluate to a single value
                None
            }
            _ => None, // For unsupported nodes
        }
    }

    pub fn eval_rgb(&self, x: f32, y: f32) -> Colour {
        if let Node::Triple(first, second, third) = self {
            let r = first.eval(x, y).unwrap_or(0.0); 
            let g = second.eval(x, y).unwrap_or(0.0);
            let b = third.eval(x, y).unwrap_or(0.0);
            Colour { r, g, b }
        } else {
            // Default to black if not a Triple node
            Colour { r: 0.0, g: 0.0, b: 0.0 }
        }
    }
    
}

#[derive(Clone)]
pub struct GrammarBranch {
    pub node: Box<Node>, 
    pub probability: f32, 
}

pub struct GrammarBranches {
    pub items: Vec<GrammarBranch>,
}

pub struct Grammar {
    pub items: Vec<GrammarBranches>, 
}

impl Grammar {
    pub fn gen_rule(&self, rule: usize, depth: i32, rng: &mut LinearCongruentialGenerator) -> Option<Box<Node>> {
        if depth <= 0 {
            return None; 
        }
    
        assert!(rule < self.items.len(), "Invalid rule index");
        let branches = &self.items[rule];
        assert!(!branches.items.is_empty(), "No branches available");
    
        let mut node = None;
    
        for _ in 0..100 { 
            let p: f32 = rng.next_float().abs(); 
    
            let mut cumulative_probability = 0.0;
            for branch in &branches.items {
                cumulative_probability += branch.probability;
                if cumulative_probability >= p {
                    node = self.gen_node(&branch.node, depth - 1, rng);
                    break;
                }
            }
    
            if node.is_some() {
                break; 
            }
        }
    
        node
    }

    fn gen_node(&self, node: &Node, depth: i32, rng: &mut LinearCongruentialGenerator) -> Option<Box<Node>> {
        match node {
            // Simple cases that can be cloned directly
            Node::X | Node::Y | Node::Number(_) | Node::Boolean(_) => Some(Box::new(node.clone())),
    
            // Unary operation (e.g., Sqrt)
            Node::Sqrt(inner) |
            Node::Sin(inner) |
            Node::Cos(inner) |
            Node::Exp(inner) => {
                let rhs = self.gen_node(inner, depth, rng)?;
                match node {
                    Node::Sqrt(_) => Some(Box::new(Node::Sqrt(rhs))),
                    Node::Sin(_) => Some(Box::new(Node::Sin(rhs))),
                    Node::Cos(_) => Some(Box::new(Node::Cos(rhs))),
                    Node::Exp(_) => Some(Box::new(Node::Exp(rhs))),
                    _ => None, // Should not reach here
                }
            }

            // Binary operations (e.g., Add, Mult, Mod, Gt, Div)
            Node::Add(lhs, rhs) |
            Node::Mult(lhs, rhs) |
            Node::Mod(lhs, rhs) |
            Node::Gt(lhs, rhs) |
            Node::Div(lhs, rhs) => {
                let lhs = self.gen_node(lhs, depth, rng)?;
                let rhs = self.gen_node(rhs, depth, rng)?;
                match node {
                    Node::Add(_, _) => Some(Box::new(Node::Add(lhs, rhs))),
                    Node::Mult(_, _) => Some(Box::new(Node::Mult(lhs, rhs))),
                    Node::Mod(_, _) => Some(Box::new(Node::Mod(lhs, rhs))),
                    Node::Gt(_, _) => Some(Box::new(Node::Gt(lhs, rhs))),
                    Node::Div(_, _) => Some(Box::new(Node::Div(lhs, rhs))),
                    _ => None, // Should not reach here
                }
            }
    
            // Triple node
            Node::Triple(first, second, third) => {
                let first = self.gen_node(first, depth, rng)?;
                let second = self.gen_node(second, depth, rng)?;
                let third = self.gen_node(third, depth, rng)?;
                Some(Box::new(Node::Triple(first, second, third)))
            }
    
            // Conditional node (If)
            Node::If { cond, then, elze } => {
                let cond = self.gen_node(cond, depth, rng)?;
                let then = self.gen_node(then, depth, rng)?;
                let elze = self.gen_node(elze, depth, rng)?;
                Some(Box::new(Node::If { cond, then, elze }))
            }
    
            // Rule node
            Node::Rule(rule_index) => self.gen_rule(*rule_index, depth - 1, rng),
    
            // Random node
            Node::Random => {
                let random_value = rng.next_float();
                Some(Box::new(Node::Number(random_value)))
            }
    
            // Default case for unsupported nodes
            _ => None,
        }
    }
}

