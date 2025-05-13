use std::fmt;
use crate::closure_tree::ClosureNode;

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
            X => writeln!(f, "{}x", pad),
            Y => writeln!(f, "{}y", pad),
            Number(n) => writeln!(f, "{}const_ ( {:?} )", pad, n),
            Random => writeln!(f, "{}random", pad),
            Rule(r) => writeln!(f, "{}rule ( {} ) ", pad, r),
            Sin(inner) => {
                writeln!(f, "{}sin ( ", pad)?;
                inner.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{} ) ", pad)
            }
            Cos(inner) => {
                writeln!(f, "{}cos ( ", pad)?;
                inner.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{} ) ", pad)
            }
            Exp(inner) => {
                writeln!(f, "{}exp ( ", pad)?;
                inner.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{} ) ", pad)
            }
            Sqrt(inner) => {
                writeln!(f, "{}sqrt ( ", pad)?;
                inner.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{} ) ", pad)
            }
            Add(a, b) => {
                writeln!(f, "{}add ( ", pad)?;
                a.fmt_pretty(f, indent + 1)?;
                b.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{} ) ", pad)
            }
            Mult(a, b) => {
                writeln!(f, "{}mult ( ", pad)?;
                a.fmt_pretty(f, indent + 1)?;
                b.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{} ) ", pad)
            }
            Div(a, b) => {
                writeln!(f, "{}div ( ", pad)?;
                a.fmt_pretty(f, indent + 1)?;
                b.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{} ) ", pad)
            }
            Modulo(a, b) => {
                writeln!(f, "{}mod ( ", pad)?;
                a.fmt_pretty(f, indent + 1)?;
                b.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{} ) ", pad)
            }
            Mix(a, b, c, d) => {
                writeln!(f, "{}mix ( ", pad)?;
                a.fmt_pretty(f, indent + 1)?;
                b.fmt_pretty(f, indent + 1)?;
                c.fmt_pretty(f, indent + 1)?;
                d.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{} ) ", pad)
            }
            MixUnbounded(a, b, c, d) => {
                writeln!(f, "{}mixu ( ", pad)?;
                a.fmt_pretty(f, indent + 1)?;
                b.fmt_pretty(f, indent + 1)?;
                c.fmt_pretty(f, indent + 1)?;
                d.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{} ) ", pad)
            }
            Triple(a, b, c) => {
                writeln!(f, "{}triple ( ", pad)?;
                a.fmt_pretty(f, indent + 1)?;
                b.fmt_pretty(f, indent + 1)?;
                c.fmt_pretty(f, indent + 1)?;
                writeln!(f, "{} ) ", pad)
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
        use Node::*;
        match self {
            Add(lhs, rhs) => {
                lhs.simplify(); 
                rhs.simplify(); 

                if let (Number(lhs_val), Number(rhs_val)) = (&**lhs, &**rhs) {
                    *self = Number((lhs_val + rhs_val)/2.0);
                }
            }
            Mult(lhs, rhs) => {
                lhs.simplify();
                rhs.simplify();

                if let (Number(lhs_val), Number(rhs_val)) = (&**lhs, &**rhs) {
                    *self = Number(lhs_val * rhs_val);
                }
            }
            Sin(inner) => {
                inner.simplify();

                if let Number(val) = **inner {
                    *self = Number(val.sin());
                }
            }
            Cos(inner) => {
                inner.simplify();

                if let Number(val) = **inner {
                    *self = Number(val.cos());
                }
            }
            Exp(inner) => {
                inner.simplify();

                if let Number(val) = **inner {
                    *self = Number(val.exp());
                }
            }
            Sqrt(inner) => {
                inner.simplify();

                if let Number(val) = **inner {
                    *self = Number(val.sqrt().max(0.0));
                }
            }
            Div(lhs, rhs) => {
                lhs.simplify();
                rhs.simplify();

                if let (Number(lhs_val), Number(rhs_val)) = (&**lhs, &**rhs) {
                    if rhs_val.abs() > 1e-6 {
                        *self = Number(lhs_val / rhs_val);
                    } else {
                        *self = Number(0.0); 
                    }
                }
            }
            Modulo(lhs, rhs) => {
                lhs.simplify();
                rhs.simplify();

                if let (Number(lhs_val), Number(rhs_val)) = (&**lhs, &**rhs) {
                    if rhs_val.abs() > 1e-6 {
                        *self = Number(lhs_val % rhs_val);
                    } else {
                        *self = Number(0.0); 
                    }
                }
            }
            Mix(a, b, c, d) => {
                a.simplify();
                b.simplify();
                c.simplify();
                d.simplify();

                if let (Number(a_val), Number(b_val),Number(c_val), Number(d_val)) = (&**a, &**b, &**c, &**d) {
                    let numerator = (a_val + 1.0) * (c_val + 1.0) + (b_val + 1.0) * (d_val + 1.0);
                    let denominator = ((a_val + 1.0) + (b_val + 1.0)).max(1e-6);
                    *self = Number((numerator / denominator) - 1.0);
                }
            }
            MixUnbounded(a, b, c, d) => {
                a.simplify();
                b.simplify();
                c.simplify();
                d.simplify();

                if let (Number(a_val), Number(b_val),Number(c_val), Number(d_val)) = (&**a, &**b, &**c, &**d) {
                    *self = Number((a_val * c_val + b_val * d_val) / (a_val + b_val + 1e-6));
                }
            }
            Number(_) | X | Y => { /* terminates recursive `simplify()` calls */}
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

    pub fn to_closure_tree(&self) -> Box<dyn ClosureNode> {
        use Node::*;
        match self {
            X => Box::new(|x, _| x),
            Y => Box::new(|_, y| y),
            Number(v) => {
                let val = *v;
                Box::new(move |_, _| val)
            }

            Add(a, b) => {
                let fa = a.to_closure_tree();
                let fb = b.to_closure_tree();
                Box::new(move |x, y| (fa(x, y) + fb(x, y)) / 2.0)
            }

            Mult(a, b) => {
                let fa = a.to_closure_tree();
                let fb = b.to_closure_tree();
                Box::new(move |x, y| fa(x, y) * fb(x, y))
            }

            Div(a, b) => {
                let fa = a.to_closure_tree();
                let fb = b.to_closure_tree();
                Box::new(move |x, y| {
                    let denom = fb(x, y);
                    if denom.abs() > 1e-6 {
                        fa(x, y) / denom
                    } else {
                        0.0
                    }
                })
            }

            Modulo(a, b) => {
                let fa = a.to_closure_tree();
                let fb = b.to_closure_tree();
                Box::new(move |x, y| {
                    let denom = fb(x, y);
                    if denom.abs() > 1e-6 {
                        fa(x, y) % denom
                    } else {
                        0.0
                    }
                })
            }

            Sqrt(inner) => {
                let f = inner.to_closure_tree();
                Box::new(move |x, y| f(x, y).sqrt().max(0.0))
            }

            Sin(inner) => {
                let f = inner.to_closure_tree();
                Box::new(move |x, y| f(x, y).sin())
            }

            Cos(inner) => {
                let f = inner.to_closure_tree();
                Box::new(move |x, y| f(x, y).cos())
            }

            Exp(inner) => {
                let f = inner.to_closure_tree();
                Box::new(move |x, y| f(x, y).exp())
            }

            Mix(a, b, c, d) => {
                let fa = a.to_closure_tree();
                let fb = b.to_closure_tree();
                let fc = c.to_closure_tree();
                let fd = d.to_closure_tree();
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

            MixUnbounded(a, b, c, d) => {
                let fa = a.to_closure_tree();
                let fb = b.to_closure_tree();
                let fc = c.to_closure_tree();
                let fd = d.to_closure_tree();
                Box::new(move |x, y| {
                    let a = fa(x, y);
                    let b = fb(x, y);
                    let c = fc(x, y);
                    let d = fd(x, y);
                    (a * c + b * d) / (a + b + 1e-6)
                })
            }

            Random => {
                panic!("Node::Random should be replaced before compilation");
            }

            Triple(_, _, _) => {
                panic!("to_closure_tree() is for scalar nodes, not Triple");
            }

            _ => unimplemented!("to_closure_tree: missing match arm for {:?}", self),
        }
    }
}