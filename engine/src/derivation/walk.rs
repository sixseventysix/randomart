use crate::derivation::rng::Rng_;
use crate::grammar::Grammar;
use crate::tree::node::Node;

pub struct Walk<'g> {
    grammar: &'g Grammar,
    rng: Rng_,
}

impl<'g> Walk<'g> {
    pub fn new(grammar: &'g Grammar, seed: u64) -> Self {
        Self {
            grammar,
            rng: Rng_::new(seed),
        }
    }

    pub fn gen_rule(&mut self, rule: usize, depth: u32) -> Option<Box<Node>> {
        if depth == 0 {
            return None;
        }

        let grammar = self.grammar;
        assert!(rule < grammar.rules.len(), "invalid rule index");
        let branches = &grammar.rules[rule];
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
