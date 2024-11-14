pub mod utils;
use utils::{Colour, LinearCongruentialGenerator};

#[derive(Clone, Debug)]
pub enum Node {
    X,                       
    Y,                       
    Random,                  
    Rule(usize),                                    // stores the index of the rule          
    Number(f32),             
    Boolean(bool),           
    Sqrt(Box<Node>),        
    Sin(Box<Node>),
    Cos(Box<Node>),
    Exp(Box<Node>),
    Add(Box<Node>, Box<Node>), 
    Mult(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Mod(Box<Node>, Box<Node>), 
    Gt(Box<Node>, Box<Node>),   
    Triple(Box<Node>, Box<Node>, Box<Node>), 
    If {
        cond: Box<Node>,     
        then: Box<Node>,    
        elze: Box<Node>,    
    },
    Mix(Box<Node>, Box<Node>, Box<Node>, Box<Node>)
}

impl Node {
    fn eval(&self, x: f32, y: f32) -> Option<f32> {
        match self {
            Node::X => Some(x),
            Node::Y => Some(y),
            Node::Number(value) => Some(*value),
            Node::Random => unreachable!("all Node::Random instances are supposed to be converted into Node::Number during generation"),
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
                Some(val.sqrt().max(0.0)) 
            }
            Node::Div(lhs, rhs) => {
                let lhs_val = lhs.eval(x, y)?;
                let rhs_val = rhs.eval(x, y)?;
                if rhs_val.abs() > 1e-6 { 
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
                unreachable!("Node::Triple is only for the Entry rule")
            }
            // todo: enforce boolean values only inside cond
            Node::If { cond, then, elze } => {
                let cond_value = cond.eval(x, y)?; 
                if cond_value > 0.0 { // non zero is true
                    then.eval(x, y)   
                } else {
                    elze.eval(x, y)   
                }
            }
            Node::Gt(lhs, rhs) => {
                let lhs_val = lhs.eval(x, y)?;
                let rhs_val = rhs.eval(x, y)?;
                Some(if lhs_val > rhs_val { 1.0 } else { 0.0 })
            }
            _ => unreachable!("unexpected Node kind during eval: {:?}", self), 
        }
    }

    pub fn eval_rgb(&self, x: f32, y: f32) -> Colour {
        if let Node::Triple(first, second, third) = self {
            let r = first.eval(x, y).unwrap_or(0.0); 
            let g = second.eval(x, y).unwrap_or(0.0);
            let b = third.eval(x, y).unwrap_or(0.0);
            Colour { r, g, b }
        } else {
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
    pub fn gen_rule(&self, rule: usize, depth: u32, rng: &mut LinearCongruentialGenerator) -> Option<Box<Node>> {
        if depth <= 0 {
            return None; 
        }
    
        assert!(rule < self.items.len(), "Invalid rule index");
        let branches = &self.items[rule];
        assert!(!branches.items.is_empty(), "No branches available");
    
        let mut node = None;
    
        for _ in 0..100 { 
            let p: f32 = rng.next_float(); 
    
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

    fn gen_node(&self, node: &Node, depth: u32, rng: &mut LinearCongruentialGenerator) -> Option<Box<Node>> {
        match node {
            Node::X | Node::Y | Node::Number(_) | Node::Boolean(_) => Some(Box::new(node.clone())),
    
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
                    _ => unreachable!("{:?} not a unary op", node), 
                }
            }

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
                    _ => unreachable!("{:?} not a binary op", node), 
                }
            }
    
            Node::Triple(first, second, third) => {
                let first = self.gen_node(first, depth, rng)?;
                let second = self.gen_node(second, depth, rng)?;
                let third = self.gen_node(third, depth, rng)?;
                Some(Box::new(Node::Triple(first, second, third)))
            }
    
            Node::If { cond, then, elze } => {
                let cond = self.gen_node(cond, depth, rng)?;
                let then = self.gen_node(then, depth, rng)?;
                let elze = self.gen_node(elze, depth, rng)?;
                Some(Box::new(Node::If { cond, then, elze }))
            }
    
            Node::Rule(rule_index) => self.gen_rule(*rule_index, depth - 1, rng),
    
            Node::Random => {
                let random_value = rng.next_float() * 2.0 - 1.0;
                Some(Box::new(Node::Number(random_value)))
            }
            Node::Mix(a, b, c, d) => {
                let a = self.gen_node(a, depth, rng)?;
                let b = self.gen_node(b, depth, rng)?;
                let c = self.gen_node(c, depth, rng)?;
                let d = self.gen_node(d, depth, rng)?;
                Some(Box::new(Node::Mix(a, b, c, d)))
            }
        }
    }
}

