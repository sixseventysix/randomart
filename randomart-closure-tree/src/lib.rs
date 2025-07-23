pub mod utils;
use utils::{ Colour, LinearCongruentialGenerator };
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    X,                       
    Y,                       
    Random,                  
    Rule(usize),                                                      // stores the index of the rule          
    Number(f32),             
    Sqrt(Box<Node>),        
    Sin(Box<Node>),
    Cos(Box<Node>),
    Exp(Box<Node>),
    Add(Box<Node>, Box<Node>), 
    Mult(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Modulo(Box<Node>, Box<Node>), 
    Triple(Box<Node>, Box<Node>, Box<Node>), 
    Mix(Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    MixUnbounded(Box<Node>, Box<Node>, Box<Node>, Box<Node>)
}

impl Node {
    pub fn fmt_pretty(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let pad = "  ".repeat(indent);
        use Node::*;
        match self {
            X => writeln!(f, "{}X", pad),
            Y => writeln!(f, "{}Y", pad),
            Number(n) => writeln!(f, "{}Const({})", pad, n),
            Random => writeln!(f, "{}Random", pad),
            Rule(r) => writeln!(f, "{}Rule({})", pad, r),
            Sin(inner) => {
                writeln!(f, "{}Sin(", pad)?;
                inner.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{})", pad)
            }
            Cos(inner) => {
                writeln!(f, "{}Cos(", pad)?;
                inner.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{})", pad)
            }
            Exp(inner) => {
                writeln!(f, "{}Exp(", pad)?;
                inner.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{})", pad)
            }
            Sqrt(inner) => {
                writeln!(f, "{}Sqrt(", pad)?;
                inner.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{})", pad)
            }
            Add(a, b) => {
                writeln!(f, "{}Add(", pad)?;
                a.fmt_pretty(f, indent + 1)?;
                b.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{})", pad)
            }
            Mult(a, b) => {
                writeln!(f, "{}Mult(", pad)?;
                a.fmt_pretty(f, indent + 1)?;
                b.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{})", pad)
            }
            Div(a, b) => {
                writeln!(f, "{}Div(", pad)?;
                a.fmt_pretty(f, indent + 1)?;
                b.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{})", pad)
            }
            Modulo(a, b) => {
                writeln!(f, "{}Modulo(", pad)?;
                a.fmt_pretty(f, indent + 1)?;
                b.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{})", pad)
            }
            Mix(a, b, c, d) => {
                writeln!(f, "{}Mix(", pad)?;
                a.fmt_pretty(f, indent + 1)?;
                b.fmt_pretty(f, indent + 1)?;
                c.fmt_pretty(f, indent + 1)?;
                d.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{})", pad)
            }
            MixUnbounded(a, b, c, d) => {
                writeln!(f, "{}MixUnbounded(", pad)?;
                a.fmt_pretty(f, indent + 1)?;
                b.fmt_pretty(f, indent + 1)?;
                c.fmt_pretty(f, indent + 1)?;
                d.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{})", pad)
            }
            Triple(a, b, c) => {
                writeln!(f, "{}Triple(", pad)?;
                a.fmt_pretty(f, indent + 1)?;
                b.fmt_pretty(f, indent + 1)?;
                c.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{})", pad)
            }
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_pretty(f, 0)
    }
}

impl Node {
    fn simplify(&mut self) {
        match self {
            Node::Add(lhs, rhs) => {
                lhs.simplify(); 
                rhs.simplify(); 

                if let (Node::Number(lhs_val), Node::Number(rhs_val)) = (&**lhs, &**rhs) {
                    *self = Node::Number((lhs_val + rhs_val)/2.0);
                }
            }
            Node::Mult(lhs, rhs) => {
                lhs.simplify();
                rhs.simplify();

                if let (Node::Number(lhs_val), Node::Number(rhs_val)) = (&**lhs, &**rhs) {
                    *self = Node::Number(lhs_val * rhs_val);
                }
            }
            Node::Sin(inner) => {
                inner.simplify();

                if let Node::Number(val) = **inner {
                    *self = Node::Number(val.sin());
                }
            }
            Node::Cos(inner) => {
                inner.simplify();

                if let Node::Number(val) = **inner {
                    *self = Node::Number(val.cos());
                }
            }
            Node::Exp(inner) => {
                inner.simplify();

                if let Node::Number(val) = **inner {
                    *self = Node::Number(val.exp());
                }
            }
            Node::Sqrt(inner) => {
                inner.simplify();

                if let Node::Number(val) = **inner {
                    *self = Node::Number(val.sqrt().max(0.0));
                }
            }
            Node::Div(lhs, rhs) => {
                lhs.simplify();
                rhs.simplify();

                if let (Node::Number(lhs_val), Node::Number(rhs_val)) = (&**lhs, &**rhs) {
                    if rhs_val.abs() > 1e-6 {
                        *self = Node::Number(lhs_val / rhs_val);
                    } else {
                        *self = Node::Number(0.0); 
                    }
                }
            }
            Node::Modulo(lhs, rhs) => {
                lhs.simplify();
                rhs.simplify();

                if let (Node::Number(lhs_val), Node::Number(rhs_val)) = (&**lhs, &**rhs) {
                    if rhs_val.abs() > 1e-6 {
                        *self = Node::Number(lhs_val % rhs_val);
                    } else {
                        *self = Node::Number(0.0); 
                    }
                }
            }
            Node::Mix(a, b, c, d) => {
                a.simplify();
                b.simplify();
                c.simplify();
                d.simplify();

                if let (Node::Number(a_val), Node::Number(b_val),Node::Number(c_val), Node::Number(d_val)) = (&**a, &**b, &**c, &**d) {
                    let numerator = (a_val + 1.0) * (c_val + 1.0) + (b_val + 1.0) * (d_val + 1.0);
                    let denominator = ((a_val + 1.0) + (b_val + 1.0)).max(1e-6);
                    *self = Node::Number((numerator / denominator) - 1.0);
                }
            }
            Node::MixUnbounded(a, b, c, d) => {
                a.simplify();
                b.simplify();
                c.simplify();
                d.simplify();

                if let (Node::Number(a_val), Node::Number(b_val),Node::Number(c_val), Node::Number(d_val)) = (&**a, &**b, &**c, &**d) {
                    *self = Node::Number((a_val * c_val + b_val * d_val) / (a_val + b_val + 1e-6));
                }
            }
            Node::Number(_) | Node::X | Node::Y => { /* terminates recursive `simplify()` calls */}
            node => {
                panic!("encountered {:?} which is not evaluatable. examine your grammar.", node)
            }
        }
    }

    pub fn simplify_triple(&mut self) {
        if let Node::Triple(first, second, third) = self {
            first.simplify(); 
            second.simplify();
            third.simplify();
        } else {
            panic!("expected Node::Triple, encountered {:?}", self);
        }
    }
}

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
                r: compile_node(r),
                g: compile_node(g),
                b: compile_node(b),
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
                if denom.abs() > 1e-6 {
                    fa(x, y) / denom
                } else {
                    0.0
                }
            })
        }

        Node::Modulo(a, b) => {
            let fa = compile_node(a);
            let fb = compile_node(b);
            Box::new(move |x, y| {
                let denom = fb(x, y);
                if denom.abs() > 1e-6 {
                    fa(x, y) % denom
                } else {
                    0.0
                }
            })
        }

        Node::Sqrt(inner) => {
            let f = compile_node(inner);
            Box::new(move |x, y| f(x, y).sqrt().max(0.0))
        }

        Node::Sin(inner) => {
            let f = compile_node(inner);
            Box::new(move |x, y| f(x, y).sin())
        }

        Node::Cos(inner) => {
            let f = compile_node(inner);
            Box::new(move |x, y| f(x, y).cos())
        }

        Node::Exp(inner) => {
            let f = compile_node(inner);
            Box::new(move |x, y| f(x, y).exp())
        }

        Node::Mix(a, b, c, d) => {
            let fa = compile_node(a);
            let fb = compile_node(b);
            let fc = compile_node(c);
            let fd = compile_node(d);
            Box::new(move |x, y| {
                let a = fa(x, y) + 1.0;
                let b = fb(x, y) + 1.0;
                let c = fc(x, y) + 1.0;
                let d = fd(x, y) + 1.0;
                let numerator = a * c + b * d;
                let denominator = (a + b).max(1e-6);
                (numerator / denominator) - 1.0
            })
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

        Node::Random => {
            panic!("Node::Random should be replaced before compilation");
        }

        Node::Triple(_, _, _) => {
            panic!("compile_node() is for scalar nodes, not Triple");
        }

        _ => unimplemented!("compile_node: missing match arm for {:?}", node),
    }
}

#[derive(Clone)]
pub struct GrammarBranch {
    pub node: Box<Node>, 
    pub probability: f32, 
}

#[derive(Clone)]
pub struct GrammarBranches {
    pub alternates: Vec<GrammarBranch>,
}

impl GrammarBranches {
    fn new() -> Self {
        Self {
            alternates: Vec::new(),
        }
    }

    fn add_alternate(&mut self, node: Node, probability: f32) {
        self.alternates.push(GrammarBranch { node: Box::new(node), probability });
    }
}

pub struct Grammar {
    pub rules: Vec<GrammarBranches>, 
    rng: LinearCongruentialGenerator
}

impl Grammar {
    fn add_rule(&mut self, branch: GrammarBranches) {
        self.rules.push(branch);
    }

    pub fn default(seed: u64) -> Self {
        let mut grammar = Self {
            rules: Vec::new(),
            rng: LinearCongruentialGenerator::new(seed),
        };

        // E::= (C, C, C)
        let mut e_branch = GrammarBranches::new();
        e_branch.add_alternate(
            Node::Triple(
                Box::new(Node::Rule(1)),
                Box::new(Node::Rule(1)),
                Box::new(Node::Rule(1)),
            ),
            1.0,
        );
        grammar.add_rule(e_branch);

        // C::= A | Add(C, C) | Mult(C, C) | Sin(C) | Cos(C) | Exp(C) | Sqrt(C) | Div(C, C) | MixUnbounded(C, C, C, C)
        let mut c_branch = GrammarBranches::new();
        c_branch.add_alternate(Node::Rule(2), 1.0 / 13.0); 
        c_branch.add_alternate(
            Node::Add(
                Box::new(Node::Rule(1)),
                Box::new(Node::Rule(1)),
            ),
            1.0 / 13.0,
        );
        c_branch.add_alternate(
            Node::Mult(
                Box::new(Node::Rule(1)),
                Box::new(Node::Rule(1)),
            ),
            1.0 / 13.0,
        );
        c_branch.add_alternate(
            Node::Sin(Box::new(Node::Rule(1))),
            3.0 / 13.0,
        );
        c_branch.add_alternate(
            Node::Cos(Box::new(Node::Rule(1))),
            3.0 / 13.0,
        );
        c_branch.add_alternate(
            Node::Exp(Box::new(Node::Rule(1))),
            1.0 / 13.0,
        );
        c_branch.add_alternate(
            Node::Sqrt(Box::new(Node::Rule(1))),
            1.0 / 13.0,
        );
        c_branch.add_alternate(
            Node::Div(
                Box::new(Node::Rule(1)),
                Box::new(Node::Rule(1)),
            ),
            1.0 / 13.0,
        );
        c_branch.add_alternate(
            Node::MixUnbounded(
                Box::new(Node::Rule(1)),
                Box::new(Node::Rule(1)),
                Box::new(Node::Rule(1)),
                Box::new(Node::Rule(1)),
            ),
            1.0 / 13.0,
        );
        grammar.add_rule(c_branch);

        // A ::= x | y | random number in [-1, 1]
        let mut a_branch = GrammarBranches::new();
        a_branch.add_alternate(Node::X, 1.0 / 3.0);
        a_branch.add_alternate(Node::Y, 1.0 / 3.0);
        a_branch.add_alternate(Node::Random, 1.0 / 3.0);
        grammar.add_rule(a_branch);

        grammar  
    
    }

    pub fn gen_rule(&mut self, rule: usize, depth: u32) -> Option<Box<Node>> {
        if depth <= 0 {
            return None; 
        }
    
        assert!(rule < self.rules.len(), "invalid rule index");
        let branches = self.rules[rule].clone();
        assert!(!branches.alternates.is_empty(), "no branches available");
    
        let mut node = None;
    
        for _ in 0..100 { 
            let p: f32 = self.rng.next_float(); 
    
            let mut cumulative_probability = 0.0;
            for branch in &branches.alternates {
                cumulative_probability += branch.probability;
                if cumulative_probability >= p {
                    node = self.gen_node(&branch.node, depth - 1);
                    break;
                }
            }
    
            if node.is_some() {
                break; 
            }
        }
    
        node
    }

    fn gen_node(&mut self, node: &Node, depth: u32) -> Option<Box<Node>> {
        match node {
            Node::X | Node::Y | Node::Number(_) => Some(Box::new(node.clone())),

            Node::Sqrt(inner) => {
                let rhs = self.gen_node(inner, depth)?;
                Some(Box::new(Node::Sqrt(rhs)))
            }
            Node::Sin(inner) => {
                let rhs = self.gen_node(inner, depth)?;
                Some(Box::new(Node::Sin(rhs)))
            }
            Node::Cos(inner) => {
                let rhs = self.gen_node(inner, depth)?;
                Some(Box::new(Node::Cos(rhs)))
            }
            Node::Exp(inner) => {
                let rhs = self.gen_node(inner, depth)?;
                Some(Box::new(Node::Exp(rhs)))
            }

            Node::Add(lhs, rhs) => {
                let lhs = self.gen_node(lhs, depth)?;
                let rhs = self.gen_node(rhs, depth)?;
                Some(Box::new(Node::Add(lhs, rhs)))
            }
            Node::Mult(lhs, rhs) => {
                let lhs = self.gen_node(lhs, depth)?;
                let rhs = self.gen_node(rhs, depth)?;
                Some(Box::new(Node::Mult(lhs, rhs)))
            }
            Node::Modulo(lhs, rhs) => {
                let lhs = self.gen_node(lhs, depth)?;
                let rhs = self.gen_node(rhs, depth)?;
                Some(Box::new(Node::Modulo(lhs, rhs)))
            }
            Node::Div(lhs, rhs) => {
                let lhs = self.gen_node(lhs, depth)?;
                let rhs = self.gen_node(rhs, depth)?;
                Some(Box::new(Node::Div(lhs, rhs)))
            }

            Node::Mix(a, b, c, d) => {
                let a = self.gen_node(a, depth)?;
                let b = self.gen_node(b, depth)?;
                let c = self.gen_node(c, depth)?;
                let d = self.gen_node(d, depth)?;
                Some(Box::new(Node::Mix(a, b, c, d)))
            }

            Node::MixUnbounded(a, b, c, d) => {
                let a = self.gen_node(a, depth)?;
                let b = self.gen_node(b, depth)?;
                let c = self.gen_node(c, depth)?;
                let d = self.gen_node(d, depth)?;
                Some(Box::new(Node::MixUnbounded(a, b, c, d)))
            }

            Node::Triple(first, second, third) => {
                let first = self.gen_node(first, depth)?;
                let second = self.gen_node(second, depth)?;
                let third = self.gen_node(third, depth)?;
                Some(Box::new(Node::Triple(first, second, third)))
            }

            Node::Rule(rule_index) => {
                let new_depth = depth.checked_sub(1)?;
                self.gen_rule(*rule_index, new_depth)
            }

            Node::Random => {
                let val = self.rng.next_float() * 2.0 - 1.0;
                Some(Box::new(Node::Number(val)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{fnv1a, PixelCoordinates, render_pixels};
    use image::RgbImage;

    fn images_are_equal(img1: &RgbImage, img2: &RgbImage) -> bool {
        if img1.dimensions() != img2.dimensions() {
            return false; 
        }
        img1.as_raw() == img2.as_raw() 
    }

    #[test]
    fn test_image_buffer_before_and_after_optimisations() {
        let (width, height) = (400, 400);
        let mut grammar = Grammar::default(fnv1a("spiderman"));
        let mut generated_node = grammar.gen_rule(0, 40).unwrap();

        let (r_node, g_node, b_node) = match &*generated_node {
            Node::Triple(r, g, b) => (r, g, b),
            _ => panic!("Expected Triple node at the top level"),
        };

        let r_fn = compile_node(&*r_node);
        let g_fn = compile_node(&*g_node);
        let b_fn = compile_node(&*b_node);
        
        let rgb_function = move |coords: PixelCoordinates| Colour {
            r: r_fn(coords.x, coords.y),
            g: g_fn(coords.x, coords.y),
            b: b_fn(coords.x, coords.y),
        };
        let img1 = render_pixels(rgb_function, width, height);
        generated_node.simplify_triple();
        let (r_node, g_node, b_node) = match &*generated_node {
            Node::Triple(r, g, b) => (r, g, b),
            _ => panic!("Expected Triple node at the top level"),
        };

        let r_fn = compile_node(&*r_node);
        let g_fn = compile_node(&*g_node);
        let b_fn = compile_node(&*b_node);
        
        let rgb_function = move |coords: PixelCoordinates| Colour {
            r: r_fn(coords.x, coords.y),
            g: g_fn(coords.x, coords.y),
            b: b_fn(coords.x, coords.y),
        };
        let img2 = render_pixels(rgb_function, width, height);
        assert!(images_are_equal(&img1, &img2));
    }
}