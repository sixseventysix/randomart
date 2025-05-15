use crate::node::Node;
use crate::utils::LinearCongruentialGenerator;

#[derive(Clone)]
struct GrammarBranch {
    node: Box<Node>, 
    probability: f32, 
}

#[derive(Clone)]
struct GrammarBranches {
    alternates: Vec<GrammarBranch>,
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

pub(crate) struct Grammar {
    rules: Vec<GrammarBranches>, 
    rng: LinearCongruentialGenerator
}

impl Grammar {
    fn add_rule(&mut self, branch: GrammarBranches) {
        self.rules.push(branch);
    }

    pub(crate) fn default(seed: u64) -> Self {
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

    pub(crate) fn gen_top_rule(&mut self, depth: u32) -> Option<Box<Node>> {
        let seed_b = self.rng.next_u64();
        let seed_c = self.rng.next_u64();
        self.rng.next_u64();

        let (b, c) = rayon::join(
            || Grammar::default(seed_b).gen_rule(1, depth - 1),
            || Grammar::default(seed_c).gen_rule(1, depth - 1),
        );
        let a = self.gen_rule(1, depth - 1);

        match (a, b, c) {
            (Some(a), Some(b), Some(c)) => Some(Box::new(Node::Triple(a, b, c))),
            _ => None,
        }
    }

    pub(crate) fn gen_rule(&mut self, rule: usize, depth: u32) -> Option<Box<Node>> {
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
            Node::Div(lhs, rhs) => {
                let lhs = self.gen_node(lhs, depth)?;
                let rhs = self.gen_node(rhs, depth)?;
                Some(Box::new(Node::Div(lhs, rhs)))
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