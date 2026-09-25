use crate::tree::node::Node;

pub(crate) struct GrammarBranch {
    pub(crate) node: Box<Node>,
    pub(crate) probability: f32,
}

pub(crate) struct GrammarBranches {
    pub(crate) alternates: Vec<GrammarBranch>,
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
    pub(crate) rules: Vec<GrammarBranches>,
}

impl Grammar {
    fn add_rule(&mut self, branch: GrammarBranches) {
        self.rules.push(branch);
    }
}

impl Default for Grammar {
    fn default() -> Self {
        let mut grammar = Self { rules: Vec::new() };

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
            Node::Add(Box::new(Node::Rule(1)), Box::new(Node::Rule(1))),
            1.0 / 13.0,
        );
        c_branch.add_alternate(
            Node::Mult(Box::new(Node::Rule(1)), Box::new(Node::Rule(1))),
            1.0 / 13.0,
        );
        c_branch.add_alternate(Node::Sin(Box::new(Node::Rule(1))), 3.0 / 13.0);
        c_branch.add_alternate(Node::Cos(Box::new(Node::Rule(1))), 3.0 / 13.0);
        c_branch.add_alternate(Node::Exp(Box::new(Node::Rule(1))), 1.0 / 13.0);
        c_branch.add_alternate(Node::Sqrt(Box::new(Node::Rule(1))), 1.0 / 13.0);
        c_branch.add_alternate(
            Node::Div(Box::new(Node::Rule(1)), Box::new(Node::Rule(1))),
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
}
