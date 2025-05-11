use crate::utils::{LinearCongruentialGenerator, Colour};

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
    fn eval(&self, x: f32, y: f32) -> f32 {
        match self {
            Node::X => x,
            Node::Y => y,
            Node::Number(value) => *value,
            Node::Random => panic!("all Node::Random instances are supposed to be converted into Node::Number during generation"),
            Node::Add(lhs, rhs) => {
                let lhs_val = lhs.eval(x, y);
                let rhs_val = rhs.eval(x, y);
                (lhs_val + rhs_val)/2.0
            }
            Node::Mult(lhs, rhs) => {
                let lhs_val = lhs.eval(x, y);
                let rhs_val = rhs.eval(x, y);
                lhs_val * rhs_val
            }
            Node::Sin(inner) => {
                let val = inner.eval(x, y);
                val.sin()
            }
            Node::Cos(inner) => {
                let val = inner.eval(x, y);
                val.cos()
            }
            Node::Exp(inner) => {
                let val = inner.eval(x, y);
                val.exp()
            }
            Node::Sqrt(inner) => {
                let val = inner.eval(x, y);
                val.sqrt().max(0.0)
            }
            Node::Div(lhs, rhs) => {
                let lhs_val = lhs.eval(x, y);
                let rhs_val = rhs.eval(x, y);
                if rhs_val.abs() > 1e-6 { 
                    lhs_val / rhs_val
                } else {
                    0.0
                }
            }
            Node::Mix(a, b, c, d) => {
                let a_val = a.eval(x, y) + 1.0;
                let b_val = b.eval(x, y) + 1.0;
                let c_val = c.eval(x, y) + 1.0;
                let d_val = d.eval(x, y) + 1.0;
                let numerator = a_val * c_val + b_val * d_val;
                let denominator = (a_val + b_val).max(1e-6);
                (numerator / denominator) - 1.0
            }
            Node::MixUnbounded(a, b, c, d) => {
                let a_val = a.eval(x, y);
                let b_val = b.eval(x, y);
                let c_val = c.eval(x, y);
                let d_val = d.eval(x, y);

                (a_val * c_val + b_val * d_val) / (a_val + b_val + 1e-6)
            }
            Node::Triple(_first, _second, _third) => {
                panic!("Node::Triple is only for the Entry rule")
            }
            Node::Modulo(lhs, rhs) => {
                let lhs_val = lhs.eval(x, y); 
                let rhs_val = rhs.eval(x, y); 
                if rhs_val.abs() > 1e-6 { 
                    lhs_val % rhs_val
                } else {
                    0.0 
                }
            }
            _ => panic!("unexpected Node kind during eval: {:?}", self), 
        }
    }

    pub fn eval_rgb(&self, x: f32, y: f32) -> Colour {
        if let Node::Triple(first, second, third) = self {
            let r = first.eval(x, y); 
            let g = second.eval(x, y);
            let b = third.eval(x, y);
            Colour { r, g, b }
        } else {
            Colour { r: 0.0, g: 0.0, b: 0.0 }
        }
    }
    
    pub fn extract_channels_as_str_from_triple(&self) -> (String, String, String) {
        assert!(
            matches!(*self, Node::Triple(_, _, _)),
            "expected the generated node to be a Node::Triple, but found: {:?}",
            self
        );
        match self {
            Node::Triple(left, middle, right) => {
                let r = format!("{:?}", left);
                let g = format!("{:?}", middle);
                let b = format!("{:?}", right);
                (r,g,b)
            }
            _ => {
                unreachable!("assert inside this function would've complained before you came here");
            }
        }
    }

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

    pub fn to_dsl_string(&self) -> String {
        match self {
            Node::X => "x".to_string(),
            Node::Y => "y".to_string(),
            Node::Number(n) => {
                if n.fract() == 0.0 {
                    format!("const_({:.1})", n) // ensures 1.0 not 1
                } else {
                    format!("const_({})", n)
                }
            },

            Node::Sqrt(inner) => format!("sqrt({})", inner.to_dsl_string()),
            Node::Sin(inner) => format!("sin({})", inner.to_dsl_string()),
            Node::Cos(inner) => format!("cos({})", inner.to_dsl_string()),
            Node::Exp(inner) => format!("exp({})", inner.to_dsl_string()),

            Node::Add(left, right) => format!("add({}, {})", left.to_dsl_string(), right.to_dsl_string()),
            Node::Mult(left, right) => format!("mul({}, {})", left.to_dsl_string(), right.to_dsl_string()),
            Node::Div(left, right) => format!("div({}, {})", left.to_dsl_string(), right.to_dsl_string()),
            Node::Modulo(left, right) => format!("modulo({}, {})", left.to_dsl_string(), right.to_dsl_string()),

            Node::Mix(a, b, c, d) => format!("mix({}, {}, {}, {})",
                a.to_dsl_string(),
                b.to_dsl_string(),
                c.to_dsl_string(),
                d.to_dsl_string(),
            ),

            Node::MixUnbounded(a, b, c, d) => format!("mixu({}, {}, {}, {})",
                a.to_dsl_string(),
                b.to_dsl_string(),
                c.to_dsl_string(),
                d.to_dsl_string(),
            ),

            _ => panic!("Unsupported node variant for DSL export"),
        }
    }
}

pub trait CompiledFn: Fn(f32, f32) -> f32 + Send + Sync {}
impl<T: Fn(f32, f32) -> f32 + Send + Sync> CompiledFn for T {}

pub fn compile_node(node: &Node) -> Box<dyn CompiledFn> {
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

    pub fn build(rules: Vec<GrammarBranches>, seed: u64) -> Self {
        Self { rules, rng: LinearCongruentialGenerator::new(seed) }
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
    
            Node::Sqrt(inner) |
            Node::Sin(inner) |
            Node::Cos(inner) |
            Node::Exp(inner) => {
                let rhs = self.gen_node(inner, depth)?;
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
            Node::Modulo(lhs, rhs) |
            Node::Div(lhs, rhs) => {
                let lhs = self.gen_node(lhs, depth)?;
                let rhs = self.gen_node(rhs, depth)?;
                match node {
                    Node::Add(_, _) => Some(Box::new(Node::Add(lhs, rhs))),
                    Node::Mult(_, _) => Some(Box::new(Node::Mult(lhs, rhs))),
                    Node::Modulo(_, _) => Some(Box::new(Node::Modulo(lhs, rhs))),
                    Node::Div(_, _) => Some(Box::new(Node::Div(lhs, rhs))),
                    _ => unreachable!("{:?} not a binary op", node), 
                }
            }
    
            Node::Triple(first, second, third) => {
                let first = self.gen_node(first, depth)?;
                let second = self.gen_node(second, depth)?;
                let third = self.gen_node(third, depth)?;
                Some(Box::new(Node::Triple(first, second, third)))
            }
    
            Node::Rule(rule_index) => {
                if let Some(new_depth) = depth.checked_sub(1) {
                    self.gen_rule(*rule_index, new_depth)
                } else {
                    None 
                }
            }
    
            Node::Random => {
                let random_value = self.rng.next_float() * 2.0 - 1.0;
                Some(Box::new(Node::Number(random_value)))
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
        }
    }
}