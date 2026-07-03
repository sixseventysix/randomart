//! Hand-built expression trees that force the math guards (div-by-~zero, sqrt of
//! negatives, mix denominator epsilon) to fire, checked two ways:
//!   1. `read_json` on the closure-tree backend produces the exact expected pixel.
//!   2. closure-tree and cranelift-jit agree byte-for-byte on the same tree.
//! The trees are constant per pixel, so every pixel has the same value; we assert
//! on pixel (0,0).

use randomart_core::node::Node;

fn num(v: f32) -> Box<Node> {
    Box::new(Node::Number(v))
}

/// A Triple whose three channels are the given scalar expressions, as JSON.
fn triple_json(r: Node, g: Node, b: Node) -> String {
    let tree = Node::Triple(Box::new(r), Box::new(g), Box::new(b));
    serde_json::to_string(&tree).expect("serialize tree")
}

/// Channel value `v` in [-1, 1] maps to a u8 the same way every backend does:
/// ((v + 1.0) * 127.5).clamp(0, 255) as u8.
fn expected_u8(v: f32) -> u8 {
    ((v + 1.0) * 127.5).clamp(0.0, 255.0) as u8
}

fn first_pixel(json: &str) -> (u8, u8, u8) {
    let out = randomart_closure_tree::read_json(json, 4, 4).unwrap();
    let d = &out.pixels.data;
    (d[0], d[1], d[2])
}

#[test]
fn div_by_zero_guard_yields_zero() {
    // Div(1, 0): |denom| = 0 is not > 1e-6, so the guard returns 0.0.
    let json = triple_json(
        Node::Div(num(1.0), num(0.0)),
        Node::Number(1.0),   // -> 255
        Node::Number(-1.0),  // -> 0
    );
    let (r, g, b) = first_pixel(&json);
    assert_eq!(r, expected_u8(0.0));
    assert_eq!(g, expected_u8(1.0));
    assert_eq!(b, expected_u8(-1.0));
}

#[test]
fn div_below_epsilon_guard_yields_zero() {
    // denom = 1e-7 < 1e-6 threshold -> guard returns 0.0 (not 1e7).
    let json = triple_json(
        Node::Div(num(1.0), num(1e-7)),
        Node::Number(0.0),
        Node::Number(0.0),
    );
    let (r, _, _) = first_pixel(&json);
    assert_eq!(r, expected_u8(0.0), "sub-epsilon divisor must be guarded to 0");
}

#[test]
fn sqrt_of_negative_clamps_to_zero() {
    // Sqrt(max(-4, 0)) = sqrt(0) = 0.
    let json = triple_json(
        Node::Sqrt(num(-4.0)),
        Node::Sqrt(num(4.0)), // sqrt(4) = 2.0 -> clamps at 255
        Node::Number(0.0),
    );
    let (r, g, _) = first_pixel(&json);
    assert_eq!(r, expected_u8(0.0));
    assert_eq!(g, expected_u8(2.0)); // (2+1)*127.5 = 382.5 -> clamp 255
    assert_eq!(g, 255);
}

#[test]
fn mix_unbounded_denominator_epsilon() {
    // a = b = 0 -> denom = 0 + 0 + 1e-6; numerator = 0 -> result 0.0, no NaN.
    let json = triple_json(
        Node::MixUnbounded(num(0.0), num(0.0), num(5.0), num(5.0)),
        Node::Number(0.0),
        Node::Number(0.0),
    );
    let (r, _, _) = first_pixel(&json);
    assert_eq!(r, expected_u8(0.0), "mix with zero weights must be finite 0");
}

/// The same adversarial trees must render identically on closure-tree and
/// cranelift-jit — the guards and math have to match across backends.
#[test]
fn backends_agree_on_guarded_trees() {
    let trees = [
        triple_json(Node::Div(num(1.0), num(0.0)), Node::Div(num(3.0), num(1e-9)), Node::Number(0.5)),
        triple_json(Node::Sqrt(num(-1.0)), Node::Sqrt(num(0.25)), Node::Exp(num(-50.0))),
        triple_json(
            Node::MixUnbounded(num(0.0), num(0.0), num(1.0), num(2.0)),
            Node::Mult(Node::Sin(num(1e-40)).into(), num(1.0)),
            Node::Number(-0.5),
        ),
    ];

    for (i, json) in trees.iter().enumerate() {
        let closure = randomart_closure_tree::read_json(json, 32, 32).unwrap();
        let jit = randomart_cranelift_jit::read_json(json, 32, 32).unwrap();
        assert_eq!(
            closure.pixels, jit.pixels,
            "closure and cranelift disagree on adversarial tree #{i}: {json}"
        );
    }
}
