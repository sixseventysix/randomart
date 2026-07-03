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
        use crate::math;
        match self {
            Sin(inner) => {
                inner.simplify();
                if let Number(val) = **inner { *self = Number(math::sinf(val)); }
            }
            Cos(inner) => {
                inner.simplify();
                if let Number(val) = **inner { *self = Number(math::cosf(val)); }
            }
            Exp(inner) => {
                inner.simplify();
                if let Number(val) = **inner { *self = Number(math::expf(val)); }
            }
            Sqrt(inner) => {
                inner.simplify();
                if let Number(val) = **inner { *self = Number(math::sqrtf(val).max(0.0)); }
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

#[cfg(test)]
mod tests {
    use super::Node::*;
    use super::*;

    fn num(v: f32) -> Box<Node> {
        Box::new(Number(v))
    }

    fn simplified(node: Node) -> f32 {
        let mut n = node;
        n.simplify();
        match n {
            Number(v) => v,
            other => panic!("expected fully-folded Number, got {other:?}"),
        }
    }

    #[test]
    fn folds_nested_constants() {
        // Add averages: (Mult(2,3)=6 , 4) -> (6+4)/2 = 5
        let tree = Add(Box::new(Mult(num(2.0), num(3.0))), num(4.0));
        assert_eq!(simplified(tree), 5.0);
    }

    #[test]
    fn div_guard_matches_runtime_formula() {
        // |denom| > 1e-6 -> normal division
        assert_eq!(simplified(Div(num(1.0), num(2.0))), 0.5);
        // denom exactly 0 and sub-epsilon -> 0.0, matching the backends' guard
        assert_eq!(simplified(Div(num(1.0), num(0.0))), 0.0);
        assert_eq!(simplified(Div(num(1.0), num(1e-7))), 0.0);
    }

    #[test]
    fn sqrt_clamps_negative_like_backends() {
        assert_eq!(simplified(Sqrt(num(4.0))), 2.0);
        assert_eq!(simplified(Sqrt(num(-4.0))), 0.0);
    }

    #[test]
    fn mix_unbounded_uses_epsilon_denominator() {
        // a=b=0 -> denom 1e-6, numerator 0 -> 0.0 (finite, no NaN)
        assert_eq!(simplified(MixUnbounded(num(0.0), num(0.0), num(5.0), num(5.0))), 0.0);
        // general case matches (a*c + b*d) / (a + b + 1e-6)
        let expect = (1.0 * 3.0 + 2.0 * 4.0) / (1.0 + 2.0 + 1e-6);
        assert_eq!(
            simplified(MixUnbounded(num(1.0), num(2.0), num(3.0), num(4.0))),
            expect
        );
    }

    #[test]
    fn leaves_x_and_y_untouched() {
        let mut n = Add(Box::new(X), num(1.0));
        n.simplify();
        // Can't fold because X is not a constant; stays an Add.
        assert!(matches!(n, Add(_, _)));
    }
}
