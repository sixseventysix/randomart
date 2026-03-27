#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Node {
    X,
    Y,
    Random,
    Rule(usize),
    Number(f32),
    Sqrt(Box<Node>),
    Sin(Box<Node>),
    Cos(Box<Node>),
    Exp(Box<Node>),
    Add(Box<Node>, Box<Node>),
    Mult(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Triple(Box<Node>, Box<Node>, Box<Node>),
    MixUnbounded(Box<Node>, Box<Node>, Box<Node>, Box<Node>),
}

impl Node {
    pub fn simplify(&mut self) {
        use Node::*;
        match self {
            Sin(inner) => {
                inner.simplify();
                if let Number(val) = **inner { *self = Number(val.sin()); }
            }
            Cos(inner) => {
                inner.simplify();
                if let Number(val) = **inner { *self = Number(val.cos()); }
            }
            Exp(inner) => {
                inner.simplify();
                if let Number(val) = **inner { *self = Number(val.exp()); }
            }
            Sqrt(inner) => {
                inner.simplify();
                if let Number(val) = **inner { *self = Number(val.sqrt().max(0.0)); }
            }
            Add(lhs, rhs) => {
                lhs.simplify();
                rhs.simplify();
                if let (Number(l), Number(r)) = (&**lhs, &**rhs) {
                    *self = Number((l + r) / 2.0);
                }
            }
            Mult(lhs, rhs) => {
                lhs.simplify();
                rhs.simplify();
                if let (Number(l), Number(r)) = (&**lhs, &**rhs) {
                    *self = Number(l * r);
                }
            }
            Div(lhs, rhs) => {
                lhs.simplify();
                rhs.simplify();
                if let (Number(l), Number(r)) = (&**lhs, &**rhs) {
                    *self = Number(if r.abs() > 1e-6 { l / r } else { 0.0 });
                }
            }
            MixUnbounded(a, b, c, d) => {
                a.simplify(); b.simplify(); c.simplify(); d.simplify();
                if let (Number(a), Number(b), Number(c), Number(d)) = (&**a, &**b, &**c, &**d) {
                    *self = Number((a * c + b * d) / (a + b + 1e-6));
                }
            }
            Number(_) | X | Y => {}
            node => panic!("encountered {:?} which is not evaluatable. examine your grammar.", node),
        }
    }

    pub fn simplify_triple(&mut self) {
        if let Node::Triple(first, second, third) = self {
            rayon::join(|| first.simplify(), || second.simplify());
            third.simplify();
        } else {
            panic!("expected Node::Triple, encountered {:?}", self);
        }
    }
}
