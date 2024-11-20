pub mod utils;
use utils::{Colour, LinearCongruentialGenerator};

#[derive(Clone, Debug, PartialEq)]
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
    Modulo(Box<Node>, Box<Node>), 
    Gt(Box<Node>, Box<Node>),   
    Triple(Box<Node>, Box<Node>, Box<Node>), 
    If(Box<Node>, Box<Node>, Box<Node>),
    Mix(Box<Node>, Box<Node>, Box<Node>, Box<Node>)
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
                let a_val = a.eval(x, y);
                let b_val = b.eval(x, y);
                let c_val = c.eval(x, y);
                let d_val = d.eval(x, y);
                (a_val * c_val + b_val * d_val) / (a_val + b_val + 1e-6)
            }
            Node::Triple(_first, _second, _third) => {
                panic!("Node::Triple is only for the Entry rule")
            }
            // todo: enforce boolean values only inside cond
            Node::If(cond, then, elze) => {
                let cond_value = cond.eval(x, y); 
                if cond_value > 0.0 { // non zero is true
                    then.eval(x, y)   
                } else {
                    elze.eval(x, y)   
                }
            }
            Node::Gt(lhs, rhs) => {
                let lhs_val = lhs.eval(x, y);
                let rhs_val = rhs.eval(x, y);
                if lhs_val > rhs_val { 1.0 } else { 0.0 }
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
            Node::Gt(lhs, rhs) => {
                lhs.simplify();
                rhs.simplify();

                if let (Node::Number(lhs_val), Node::Number(rhs_val)) = (&**lhs, &**rhs) {
                    *self = Node::Number(if lhs_val > rhs_val { 1.0 } else { -1.0 });
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
            Node::If(cond, then, elze) => {
                cond.simplify();
                then.simplify();
                elze.simplify();

                if let Node::Number(cond_val) = **cond {
                    if cond_val > 0.0 {
                        *self = (**then).clone(); 
                    } else {
                        *self = (**elze).clone(); 
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

        // C::= A | Add(C, C) | Mult(C, C) | Sin(C) | Cos(C) | Exp(C) | Sqrt(C) | Div(C, C) | Mix(C, C, C, C)
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
            Node::Mix(
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
            Node::X | Node::Y | Node::Number(_) | Node::Boolean(_) => Some(Box::new(node.clone())),
    
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
            Node::Gt(lhs, rhs) |
            Node::Div(lhs, rhs) => {
                let lhs = self.gen_node(lhs, depth)?;
                let rhs = self.gen_node(rhs, depth)?;
                match node {
                    Node::Add(_, _) => Some(Box::new(Node::Add(lhs, rhs))),
                    Node::Mult(_, _) => Some(Box::new(Node::Mult(lhs, rhs))),
                    Node::Modulo(_, _) => Some(Box::new(Node::Modulo(lhs, rhs))),
                    Node::Gt(_, _) => Some(Box::new(Node::Gt(lhs, rhs))),
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
    
            Node::If(cond, then, elze) => {
                let cond = self.gen_node(cond, depth)?;
                let then = self.gen_node(then, depth)?;
                let elze = self.gen_node(elze, depth)?;
                Some(Box::new(Node::If(cond, then, elze)))
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{fnv1a, PixelCoordinates, render_pixels};
    use image::RgbImage;

    #[test]
    fn test_thumbnail_image() {
        let mut grammar = Grammar::default(fnv1a("spiderman"));
        let generated_node = grammar.gen_rule(0, 40).unwrap();
        let (r_str, g_str, b_str) = generated_node.extract_channels_as_str_from_triple();

        assert_eq!(r_str.as_str(), "Div(Add(Cos(Number(0.8143064)), Sin(Cos(Mult(Y, Div(Sin(Sin(X)), Mult(Cos(Cos(Exp(Sin(Cos(Y))))), Sin(Sin(Sqrt(Div(Sqrt(Sin(Exp(Mult(Sin(Sqrt(X)), Sin(Sin(X)))))), Sin(Div(Sin(Add(Sqrt(Cos(X)), Sin(Sqrt(X)))), Exp(Sin(Div(Exp(Number(0.65621984)), Div(X, Y)))))))))))))))), Exp(Div(Cos(Sin(Cos(Sqrt(Number(-0.4636864))))), Mix(Mult(Mult(Sin(Mult(Mix(Mult(Sin(Sin(Number(-0.3169167))), Div(Mult(Sin(Cos(Cos(Sin(Cos(X))))), Add(Exp(Cos(Cos(Sin(Number(0.1145941))))), Add(Sin(Sin(Mult(Y, Number(0.55249023)))), Cos(Sqrt(Sin(X)))))), Add(Sin(Mix(Sin(Mult(Mix(Number(-0.2570064), X, X, Y), Sqrt(X))), Exp(Mix(Sin(X), Sin(Y), Cos(X), Mix(Y, X, Number(-0.85492814), X))), Exp(Cos(Sin(X))), Sin(Div(Exp(Y), Add(Number(0.39193344), X))))), X))), Sqrt(Number(-0.43099332)), Add(Exp(Mult(Mix(Div(Sin(Sqrt(Div(Y, Y))), Cos(Exp(Sin(X)))), Cos(Div(Mix(Mult(Number(-0.9781154), Number(0.98348093)), Y, Mix(X, Number(0.9829658), X, X), Add(Number(0.00033164024), Y)), Div(Add(X, Number(0.37760782)), Cos(Number(-0.26082957))))), Number(-0.3052044), Cos(Number(-0.28564852))), Mult(Cos(Sin(Div(Cos(X), Number(0.734452)))), Cos(Cos(Y))))), Cos(X)), Mult(Add(Sqrt(Sin(Sin(Sin(Sin(Sin(X)))))), Div(Sin(Cos(Exp(Cos(Mult(Number(0.28140485), Number(0.46307325)))))), Sin(Sin(Sqrt(Div(Add(Y, X), Sin(Number(0.39548683)))))))), Cos(Sqrt(Add(Sin(Sin(Sin(Sin(Y)))), Mix(Mix(Exp(Mix(X, Number(0.6764331), Number(-0.002668023), Y)), Number(-0.6547586), Exp(X), Exp(Add(Number(0.2886889), Y))), Sin(Number(0.32838047)), Cos(Sin(Cos(Number(-0.3964551)))), Mix(Exp(Cos(Y)), Y, Add(Add(Y, X), Sin(X)), Exp(Sin(X))))))))), Add(Div(Mix(X, Div(X, Add(Exp(Cos(Sin(Number(0.5042167)))), Sin(Sqrt(Number(0.38737178))))), Exp(Sin(Exp(Sin(Sqrt(Sin(Number(-0.5439882))))))), Exp(Div(Sqrt(Sqrt(Number(0.6149641))), Sin(Sin(Y))))), Add(Mult(Mix(Sqrt(Mix(Exp(Cos(Number(0.5883217))), Sqrt(Exp(Y)), X, Mult(Sin(Number(0.78537667)), Sin(Number(0.7132455))))), Mult(Div(Cos(Cos(Number(0.3556627))), Sin(Sqrt(X))), Cos(Sin(Cos(X)))), Number(-0.16642624), Sin(Sin(Mult(Number(0.6821568), Sin(Number(-0.74198234)))))), Cos(Sqrt(Number(-0.15647155)))), Sin(Sin(Cos(Cos(Cos(Sin(Y)))))))), Div(Mult(Sin(Mult(Sin(Exp(Cos(Mix(X, X, Y, Number(-0.26491487))))), Number(-0.5365895))), Sin(Div(Add(Mix(Div(Exp(Number(-0.0028839111)), Cos(X)), Sin(Cos(X)), Sin(Cos(Number(0.27534556))), Cos(Mult(Y, X))), Cos(Mix(Sin(Number(0.8018645)), Cos(X), Div(X, Y), Cos(Y)))), X))), Exp(Sin(Sin(Mix(Mult(Div(Sqrt(Y), Number(0.14593363)), Sqrt(Sin(X))), Cos(Cos(Mult(Number(0.17649806), X))), Mult(Mult(Exp(Number(0.57681966)), Number(0.8129909)), Cos(Div(X, Y))), Sin(X))))))))), Cos(Sin(Exp(Cos(Mult(Cos(Add(Cos(Cos(Add(Sqrt(Number(-0.1577245)), Cos(Number(0.7597283))))), Add(Add(Mix(Sin(Y), Cos(Y), Cos(Y), Sin(Y)), Mult(Mix(X, X, Number(-0.8217878), Number(-0.8065264)), X)), Cos(Sin(Sin(Y)))))), Cos(Cos(Sin(Sin(Y)))))))))), Div(Cos(Y), Cos(Y))), Cos(Mult(Sin(Sqrt(Exp(Sqrt(Cos(Add(Mult(Cos(Mix(Number(-0.4547348), Y, Number(-0.40319186), Sin(Sin(Y)))), Sqrt(Exp(Sin(X)))), Cos(Sin(Div(Cos(Sin(Number(0.18493366))), Add(Mult(Number(0.065757394), Y), Sqrt(X))))))))))), Cos(Cos(Mult(Cos(Mult(Sin(Cos(Div(Cos(Sin(Mult(X, X))), Cos(Mult(Cos(X), Cos(X)))))), Cos(Cos(Cos(Y))))), Exp(Cos(Sqrt(Cos(Mult(Sin(Mult(Div(Number(-0.7573502), Number(-0.9857584)), Mix(X, Y, Y, X))), Sqrt(Mult(Cos(Y), Div(Y, X))))))))))))), Cos(Add(Sin(Cos(Sin(Add(Mult(Exp(X), Sqrt(Exp(Mult(Exp(Mult(Add(Number(-0.31604564), Number(-0.7738019)), Sin(Y))), Add(Cos(Add(Y, Y)), Cos(Sin(Number(-0.15715218)))))))), Div(Add(Cos(Cos(Div(Sin(X), Cos(Cos(Number(0.24509537)))))), Sin(Cos(Sin(Sin(X))))), Add(Cos(Cos(Sin(Exp(Cos(Y))))), Cos(Add(Sin(Add(Sqrt(X), Div(Y, X))), Sqrt(Sin(X)))))))))), Sqrt(Sqrt(Add(Add(Sin(Cos(Sin(Exp(Cos(Mult(Cos(Y), Add(Y, Number(0.20476437)))))))), Cos(Sin(Cos(Y)))), Cos(Mix(Div(Cos(Mult(Sin(Add(Sin(Y), Add(Number(-0.024786115), Number(-0.2799965)))), Add(Div(Mult(X, Number(-0.4414025)), Mix(Number(-0.9442846), Y, Y, X)), Sin(Cos(Number(-0.8271515)))))), Cos(Add(Sin(Cos(Sin(X))), Div(Exp(Sin(X)), Exp(Cos(X)))))), Sin(Div(Cos(Cos(Sin(Sqrt(Number(0.23292065))))), Div(Sin(Cos(Mult(X, Y))), X))), Exp(Exp(Sin(Mult(Cos(Sin(Number(-0.3700946))), Div(Cos(Number(0.7585335)), Cos(X)))))), Cos(Sqrt(Div(Sqrt(Add(Sin(X), Cos(Number(-0.34130484)))), Cos(Mult(Sqrt(Number(0.50590134)), Cos(Number(-0.36470628)))))))))))))), Sin(Sin(Mix(X, Sin(Mix(Exp(Sqrt(Mix(Mix(Sin(Sin(Sin(Sin(Number(-0.34478718))))), Sin(Sin(Mult(Add(Y, Number(-0.045727193)), Exp(Number(0.29553854))))), Sin(Cos(Exp(Add(Number(0.4138062), Y)))), Cos(Mix(Add(Cos(X), Sqrt(Y)), Exp(Sin(Number(-0.04634434))), Add(Cos(X), Cos(Number(0.2695595))), Number(0.66356707)))), Exp(Sin(Sqrt(Mult(Cos(Number(-0.9506049)), Div(Number(-0.9438669), Number(0.65550923)))))), Cos(Cos(Cos(Sin(Div(Y, Number(-0.114050984)))))), Sin(Y)))), Add(Cos(Mix(Sin(Cos(Mix(Cos(Mult(Y, Y)), Mix(Cos(Y), X, Sqrt(Number(-0.22931701)), Sqrt(Y)), Exp(Add(Y, X)), Mult(Add(Y, X), Add(Number(-0.3215127), Y))))), Number(0.24276114), Exp(Cos(Sqrt(Sin(Cos(Number(-0.18957245)))))), Cos(Sin(Cos(Cos(Exp(X))))))), Mult(Mult(Sin(Cos(Add(Cos(Sin(Number(0.7906107))), Sin(Sin(Number(0.84752166)))))), Mult(Cos(Div(Sqrt(Div(Y, Number(-0.64992726))), Cos(Cos(Number(-0.4923606))))), Sin(Sqrt(Exp(Add(X, Y)))))), Sqrt(Y))), Cos(Sin(Div(Mult(Mult(Cos(Add(Add(Number(-0.62430465), Y), Number(0.31052673))), Cos(Sqrt(Sin(Y)))), Cos(Sqrt(Cos(Add(Number(-0.07915354), Number(-0.28086126)))))), Sin(Mix(Sin(Number(-0.3475607)), Mix(Exp(Sqrt(Number(-0.8519582))), Div(Sin(X), Cos(Number(0.69957685))), Sin(X), Sin(Mix(Number(-0.32548696), X, Y, Y))), X, Cos(Add(Cos(Y), Y))))))), Sqrt(Cos(Sin(Add(Mult(Cos(Mult(Sin(Y), Sin(Y))), Sin(Exp(Cos(Y)))), Cos(Mix(Cos(Mult(X, Y)), Cos(Cos(Y)), Mix(X, Y, Exp(Number(0.9540616)), Cos(Number(0.120253205))), Y)))))))), Sin(Cos(Cos(Sin(Add(Sin(Number(0.44129848)), Mix(Sin(Add(Mix(Add(Number(0.9994931), Y), Div(X, Y), Cos(Number(0.98042333)), Sin(Y)), Sin(Sin(Number(-0.59432995))))), Mult(Exp(Mult(Add(Number(0.92121184), X), Sqrt(Number(0.31801617)))), Cos(Div(Sin(Y), Sqrt(Number(-0.709242))))), Sin(Sin(Div(Sin(X), Sin(X)))), Sqrt(Mult(Sqrt(Mult(X, Number(0.04581046))), Add(Y, Y))))))))), Cos(Cos(Sin(Mix(Div(Add(Cos(Exp(Sin(Add(Y, X)))), Cos(Mult(Mix(Sin(Y), Exp(X), Cos(Y), Div(Number(0.8831701), Y)), Sin(Sin(X))))), X), Number(-0.41597492), Sin(Cos(Sqrt(Cos(Cos(Cos(Number(-0.76706165))))))), Add(Exp(Cos(Exp(Div(Mix(Number(0.18548024), Number(-0.04697287), X, Number(0.06729615)), Sqrt(X))))), Mix(Mix(Sin(Sin(Cos(X))), Mult(Sqrt(Mult(X, Number(0.8523488))), Exp(Mult(Y, Y))), Mix(Add(X, Cos(X)), Sin(Div(Y, Number(-0.0379979))), Cos(Cos(X)), Sin(Cos(Number(0.8349887)))), Cos(Cos(Exp(X)))), Mix(Cos(Sin(Div(X, X))), Sqrt(Y), Cos(Exp(Cos(X))), Cos(Div(X, Number(-0.17966276)))), Add(Cos(Number(0.7713872)), Sin(Sin(Sin(Number(-0.051343262))))), Cos(Sin(Cos(Cos(Y)))))))))))))))))");

        assert_eq!(g_str.as_str(), "Y");

        assert_eq!(b_str.as_str(), "Mult(Cos(Sqrt(Mix(Sin(Cos(Sin(Cos(Mix(Sin(Sin(Cos(Sin(Cos(Exp(Cos(Cos(X)))))))), Exp(Cos(Sin(Cos(Sin(Sqrt(Mult(Add(X, Y), Cos(Y)))))))), Cos(Cos(Sqrt(Add(Mult(Sqrt(Sin(Y)), Exp(Mult(Sin(Number(-0.6905869)), Add(Y, Y)))), Mix(Sin(Mult(Add(Number(-0.35400218), Y), Exp(Y))), Div(Div(Cos(X), Mult(X, X)), Mult(Cos(Y), Add(Y, Number(0.6860547)))), Sin(Sin(Sin(Y))), Cos(Cos(Mult(Number(0.6749203), Y)))))))), Sin(Cos(Sqrt(Add(X, Mix(Sin(Add(Sqrt(X), Exp(Y))), Number(0.27389026), Sin(Add(Add(Y, Y), Mix(Number(0.60576737), X, X, Number(-0.8773289)))), Sqrt(Mix(Sin(Y), Number(-0.07104665), Sin(X), Add(Number(0.40966177), Y))))))))))))), Number(-0.6661038), Cos(Sin(Sqrt(Sin(Cos(X))))), Sin(Cos(Cos(X)))))), Mix(Mult(Y, Mult(Cos(Add(Exp(Mult(Mix(Add(Sqrt(Mult(Cos(Mult(Sin(Sin(Add(Number(0.017221093), Number(-0.35835278)))), Mult(Add(Sin(Number(0.60767245)), Exp(Y)), Sin(Mult(X, X))))), Number(0.9294585))), Mult(Cos(Sqrt(Number(0.8086232))), Cos(Add(Sin(Cos(Cos(Cos(Y)))), Sqrt(Cos(Cos(Sin(X)))))))), Mix(Mult(Cos(Sqrt(Cos(Cos(Sin(Cos(X)))))), Sqrt(Cos(Sin(Cos(Sqrt(Sin(Number(-0.04925102)))))))), Mix(Number(-0.88410664), Mix(Sin(Add(Mix(Sqrt(Sin(Y)), Mix(Sin(X), Sin(Y), Cos(Y), Cos(X)), Sin(Sin(Number(-0.13401073))), Cos(Exp(Number(-0.85643446)))), Cos(Mix(Sin(Number(0.8082372)), Sin(Y), Sqrt(Y), Sqrt(Number(-0.11469716)))))), Cos(Sqrt(Cos(Sqrt(Sin(X))))), Cos(Sqrt(Cos(Sin(Add(Number(-0.12995744), X))))), Div(Cos(Cos(Sqrt(Cos(X)))), Sqrt(Sin(Mix(Sqrt(Number(0.3694166)), Sin(Y), Sin(Y), Cos(Y)))))), Y, Exp(Cos(Cos(Div(Cos(Sin(Number(-0.057269216))), Exp(Sin(Y))))))), Add(Exp(Cos(Exp(Sqrt(Exp(Sin(Y)))))), Cos(Mix(Sin(Sin(Sin(Mix(Number(0.750396), Y, Number(0.7005997), X)))), Mult(Sin(Mult(Cos(X), Exp(Number(0.3178023)))), Cos(X)), Exp(Exp(Sin(Sqrt(Number(0.08162284))))), Sin(Add(Mix(Sqrt(Y), Y, Cos(Number(0.13400638)), Mult(X, Number(0.30471373))), Cos(Div(Number(-0.5521328), X))))))), Mix(Sin(Sin(Sqrt(Mix(Cos(Exp(Y)), Add(Cos(Y), Mult(Y, X)), Cos(Sqrt(Number(0.35937166))), Sin(Cos(Y)))))), Sqrt(Cos(Mix(Cos(Cos(Cos(Y))), Sin(Mix(Add(X, X), Mult(Number(0.08909309), Y), Add(X, Number(0.5878979)), Add(X, Number(-0.106450975)))), Cos(Mult(Number(0.4321984), Sin(X))), Div(Div(Sin(Number(0.38235152)), Cos(Y)), Cos(Add(X, Number(-0.4566828))))))), Sqrt(Cos(Number(0.9781145))), Exp(Cos(Cos(Mix(Sin(Mult(Y, Number(-0.59669995))), Exp(Exp(Y)), Sqrt(Div(Number(-0.6026432), Y)), Exp(Cos(Number(0.7516569))))))))), Cos(Cos(Sqrt(Cos(Number(0.85662365))))), Cos(Div(Cos(Cos(Mix(X, Sin(Y), Cos(Exp(Exp(X))), Div(Y, Cos(Mult(Y, Number(-0.6366973))))))), Sin(Sin(Mix(Cos(Y), Div(Mult(Sin(Number(-0.06451988)), Sin(X)), Cos(Add(Number(0.34597528), Y))), Cos(Cos(Sin(Y))), Mix(Add(Mult(Number(-0.8950111), Y), Sin(X)), Y, Sin(Cos(Number(-0.59043324))), Sin(Cos(Y))))))))), Div(Cos(Cos(Cos(Cos(Sin(Mult(Cos(Sin(Y)), Cos(Sin(Number(-0.2986315))))))))), Mix(Cos(Cos(Sin(Cos(Sin(Sin(Cos(Number(-0.7747871)))))))), Cos(Exp(Sqrt(Sin(Sin(Cos(Cos(Y))))))), Cos(Cos(Mult(Mix(Mix(Cos(Sin(X)), Mult(Cos(Number(-0.6629625)), Div(Y, X)), Mult(Add(Number(0.88850343), Y), Sin(Y)), Exp(Sin(Number(-0.8826435)))), Cos(Cos(Cos(X))), Sin(Cos(Mix(Y, X, Number(0.2969669), Number(0.6180321)))), Exp(Cos(Number(0.4697491)))), Sin(Div(Mix(Mix(Y, Y, X, Y), Mult(X, Number(0.38502717)), Sin(Number(0.06853664)), Mult(Number(-0.5732194), Y)), Sin(Div(Number(-0.0726378), X))))))), Sin(Sin(Exp(Number(-0.9310025)))))))), Sin(Mix(Cos(Cos(Sin(Sqrt(Y)))), Mult(Sqrt(Mult(Mult(Sin(Add(Sin(Sin(Sin(Number(0.33570385)))), Cos(Y))), Cos(Y)), Cos(Cos(Mix(Cos(Cos(Mix(X, Y, Y, Number(-0.50199676)))), Cos(Div(Sin(Y), Cos(X))), Mult(Sin(Div(Y, X)), Exp(Cos(Y))), X))))), Cos(Mix(Sin(Cos(Sqrt(Sin(Mix(Div(X, Number(-0.33304155)), Exp(Number(0.22646868)), Add(X, X), X))))), Sin(Div(Cos(Cos(Sin(Sin(Y)))), Sin(Exp(Add(X, Cos(X)))))), Div(Mult(Sqrt(Mix(Y, Cos(Sqrt(Number(0.46237206))), Exp(Number(-0.45443255)), Cos(Cos(Y)))), Cos(X)), Sin(Mult(Sin(Mult(Mult(Number(0.53478885), Y), Add(Number(-0.5477965), Y))), Sin(Cos(Cos(X)))))), Cos(Cos(Cos(Sin(Cos(Sin(Number(-0.14389837)))))))))), Cos(Sqrt(Add(Sin(Sin(Y)), Exp(X)))), Add(Exp(Exp(Add(Sqrt(Sin(Cos(Mult(Exp(Number(0.8881458)), Mult(X, Number(0.91461563)))))), X))), Exp(Sin(Mix(Div(Exp(Mult(Sqrt(Add(Y, Number(-0.8905563))), Mult(Add(X, Y), Exp(Number(0.7422905))))), Sin(Sqrt(Sin(Div(Y, Y))))), Cos(Sqrt(Y)), Exp(X), Sin(Cos(Cos(Sin(Exp(Number(0.9647409)))))))))))))), Sin(Cos(Mult(Mult(Cos(Number(0.81046116)), Add(Cos(Cos(Mult(Sqrt(Cos(Sin(Mult(Cos(Y), X)))), Mix(Cos(Exp(Mult(Cos(X), Cos(Y)))), Mix(Div(Cos(Mix(Y, Y, Number(-0.3753299), X)), Sin(Cos(Number(0.411824)))), Sin(Sin(Sin(Number(0.51446164)))), Div(Cos(Exp(X)), Sin(Add(Number(-0.20847046), Number(0.010793686)))), Mix(Exp(Exp(X)), Add(Cos(X), Cos(X)), Sin(Number(-0.28939295)), Div(Sin(Y), Mult(Number(-0.7275826), Number(-0.44477695))))), Cos(Add(Add(Mix(Y, X, Y, X), Sqrt(X)), Exp(Mix(Number(-0.79582965), Number(0.03277147), Y, Number(0.7603396))))), Sqrt(Div(X, Mult(Sin(Y), Sin(X)))))))), Sin(Cos(Div(Cos(Sin(Cos(Add(Cos(X), Exp(Number(-0.83072627)))))), Sin(Div(Mix(Exp(Cos(Number(-0.05835992))), Mix(Exp(Y), Exp(X), Sin(X), Mult(X, X)), Mix(Sqrt(X), Div(Number(0.1258074), Number(-0.39264464)), Sin(Y), Cos(Y)), Cos(X)), Mult(Cos(Exp(X)), Mult(Sin(Y), Cos(Y)))))))))), Exp(Cos(Mult(Mult(Sin(Mult(Add(Div(Add(Add(X, Number(-0.013236105)), Sqrt(Y)), Y), Sin(Mix(Sin(X), Add(X, X), Number(0.47213364), Div(Y, X)))), Mult(Mult(Sin(Sin(X)), Cos(Sin(X))), Cos(Number(0.041761756))))), Cos(Div(Div(Sin(X), Sin(Cos(Sin(X)))), Cos(X)))), Sin(Sin(Exp(Mix(Mult(Mix(Cos(X), Exp(Number(0.84722567)), Div(X, Number(0.901212)), Cos(Number(0.3321947))), Div(Sin(Y), Sqrt(Number(0.92377603)))), Div(Cos(Exp(Number(-0.15175617))), Sqrt(Div(Y, Number(0.17022884)))), Sqrt(Div(Sin(X), Cos(X))), Sin(Cos(Div(X, Y))))))))))))))), Sin(Sin(Y)), Exp(Sin(Number(0.75284636))), Cos(Exp(Sqrt(Mix(Add(Sin(Sin(Div(Mult(Mix(Sin(Sin(Div(Sin(Cos(Number(0.8486358))), Cos(Sin(Y))))), Cos(Cos(Div(Sin(Exp(Number(-0.15123808))), Mix(Sqrt(Number(-0.4301167)), Exp(Number(0.71335006)), Mult(X, X), Cos(X))))), Sin(Div(Sin(Y), Sin(Div(Cos(X), Cos(X))))), Sin(Add(Cos(Cos(Cos(Y))), Exp(Sin(Mult(Number(-0.5917237), X)))))), Cos(Cos(Sin(Y)))), Mix(Cos(Y), Sqrt(Mult(Cos(Cos(Sqrt(Cos(X)))), Cos(Sqrt(Cos(Add(X, Number(0.6104591))))))), Div(Cos(Div(Sin(X), Add(Div(Cos(Y), Sin(X)), Sin(Mult(X, Y))))), Mix(Mult(Mix(Exp(Number(-0.65327084)), Mix(Sin(X), Cos(Y), Sqrt(Number(0.9483335)), Add(Number(0.582808), Y)), Mult(Cos(X), Cos(Y)), Sin(Sqrt(X))), Sqrt(Exp(Y))), Div(Cos(Y), Exp(X)), Add(Mix(Sin(Sin(X)), Div(Sin(Number(0.20206654)), Add(X, Y)), Add(Exp(Y), Exp(Y)), Y), Cos(Sin(Cos(Y)))), Mult(Mix(Sin(Sin(X)), Add(Add(Number(0.79266167), X), Sin(Number(-0.16832817))), Cos(X), Add(Exp(X), Sqrt(X))), Sin(Sin(Cos(X)))))), Cos(Sin(Sin(Cos(Div(Y, Div(Y, X)))))))))), Add(Cos(Cos(Sin(Sin(Div(Cos(Sin(Add(Mix(Y, Number(0.45666337), X, X), Exp(Number(-0.6367231))))), Mix(Cos(Mult(Number(0.17502296), Exp(Number(0.15866947)))), Sin(Sin(Cos(Y))), Div(Add(Sin(X), Cos(Y)), Cos(Sin(Y))), Cos(Div(Cos(Y), Cos(Y))))))))), Mult(Sqrt(Add(Sin(Cos(Div(Div(Cos(Mix(X, X, Y, X)), Cos(Sqrt(X))), Mix(Cos(Mult(Number(-0.38597637), Y)), Mult(Div(Y, X), Mult(X, Number(-0.5198046))), Div(Sin(Number(0.051602006)), Mult(X, X)), Sin(Sin(X)))))), Cos(Mult(Cos(X), X)))), Sqrt(Sin(Div(Add(Cos(Div(Number(0.74073565), Mult(Cos(Number(0.5913513)), Sin(Number(0.18268895))))), Mix(Cos(Mult(Cos(Number(0.8041284)), Cos(X))), X, X, X)), Y)))))), Cos(Sin(Sin(Sin(Sin(Exp(Mult(Mix(Mix(Cos(Exp(Number(-0.69172716))), Add(Cos(X), Exp(Y)), Mult(Cos(X), Sin(Y)), Add(Sin(Number(-0.15534788)), Cos(X))), Div(X, Add(X, Mult(X, Number(0.23911572)))), Exp(Mult(Div(Number(0.4563409), Number(-0.8205413)), Mix(X, X, Number(0.08039367), Number(0.5718888)))), Sin(Add(Number(0.9864645), Add(X, Y)))), Exp(Cos(X))))))))), Sin(Div(Exp(Cos(Sin(Sin(Sin(Mult(Number(0.5985373), Div(Div(Sin(X), Mult(X, Number(0.10399163))), Y))))))), Cos(Sin(Div(Sin(Sqrt(Number(0.95729506))), Cos(Div(Mult(Sin(Cos(Sin(Y))), Add(Mix(Cos(Y), Sin(X), Mix(Y, Number(-0.86337614), X, Number(-0.09413463)), Y), Add(Mult(Number(0.6621188), Number(-0.97450554)), Sin(Y)))), Mult(Cos(Sin(X)), Div(Cos(Cos(X)), Mult(Cos(Y), Cos(Number(-0.40419537)))))))))))), Add(Mult(Y, Y), Sqrt(Div(Cos(Cos(Div(Exp(Cos(Sin(Mult(Mix(Y, X, Y, Y), Add(X, Number(0.763054)))))), Mix(Sin(Mix(Exp(Mix(Number(-0.92663693), Number(0.07739127), Y, Number(-0.59588516))), Div(Mix(Number(-0.7822008), Y, Number(-0.7499936), Number(-0.4514976)), Mult(X, X)), Mix(Div(Number(-0.5781597), X), Add(Number(-0.9275126), X), Sin(Y), Sqrt(Number(0.9521005))), Sin(Sin(X)))), Exp(Sin(Mix(Sin(Y), Cos(Y), Cos(Number(-0.025647998)), Div(Y, X)))), Cos(Cos(Exp(Cos(Y)))), Mix(Cos(Mult(Mix(X, Number(0.6559305), X, Y), Sqrt(Y))), Sin(Cos(Cos(X))), Exp(Cos(Sin(Y))), Sin(Number(0.38516438))))))), Cos(Sqrt(Cos(Cos(Cos(Mult(Y, Cos(Cos(X)))))))))))))))))");
    }

    fn images_are_equal(img1: &RgbImage, img2: &RgbImage) -> bool {
        if img1.dimensions() != img2.dimensions() {
            return false; 
        }
        img1.as_raw() == img2.as_raw() 
    }

    #[test]
    fn test_image_buffer_before_and_after_optimisations() {
        let mut grammar = Grammar::default(fnv1a("spiderman"));
        let mut generated_node = grammar.gen_rule(0, 40).unwrap();

        let rgb_function = |coords: PixelCoordinates| {
            generated_node.eval_rgb(coords.x, coords.y)
        };

        let img1 = render_pixels(rgb_function, 400, 400);
        let (r_str, g_str, b_str) = generated_node.extract_channels_as_str_from_triple();

        generated_node.simplify_triple();

        let rgb_function = |coords: PixelCoordinates| {
            generated_node.eval_rgb(coords.x, coords.y)
        };

        let img2 = render_pixels(rgb_function, 400, 400);
        let (r_str_optimised, g_str_optimised, b_str_optimised) = generated_node.extract_channels_as_str_from_triple();

        assert!(images_are_equal(&img1, &img2));
        assert_eq!(r_str.len() - r_str_optimised.len(), 1151);
        assert_eq!(g_str.len() - g_str_optimised.len(), 0);
        assert_eq!(b_str.len() - b_str_optimised.len(), 924);
    }

    #[test]
    #[should_panic(expected = "expected the generated node to be a Node::Triple")]
    fn test_extract_channels_from_triple_panics_on_invalid_variant() {
        let invalid_node = Node::X;
        invalid_node.extract_channels_as_str_from_triple();
    }
}

