use crate::node::Node;
use std::fmt::Write;

struct CodegenCtx {
    lines: Vec<String>,
    counter: usize,
}

impl CodegenCtx {
    pub fn new() -> Self {
        Self { lines: Vec::new(), counter: 0 }
    }

    fn next_tmp(&mut self) -> String {
        let name = format!("t{}", self.counter);
        self.counter += 1;
        name
    }

    fn emit(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn gen(&mut self, node: &Node) -> String {
        match node {
            Node::X => "x".to_string(),
            Node::Y => "y".to_string(),

            Node::Number(n) => {
                let tmp = self.next_tmp();
                self.emit(format!("float {} = {:.6};", tmp, n));
                tmp
            }

            Node::Sin(inner) => {
                let arg = self.gen(inner);
                let tmp = self.next_tmp();
                self.emit(format!("float {} = sin({});", tmp, arg));
                tmp
            }

            Node::Cos(inner) => {
                let arg = self.gen(inner);
                let tmp = self.next_tmp();
                self.emit(format!("float {} = cos({});", tmp, arg));
                tmp
            }

            Node::Sqrt(inner) => {
                let arg = self.gen(inner);
                let tmp = self.next_tmp();
                self.emit(format!("float {} = sqrt(fmax({}, 0.0));", tmp, arg));
                tmp
            }

            Node::Exp(inner) => {
                let arg = self.gen(inner);
                let tmp = self.next_tmp();
                self.emit(format!("float {} = exp({});", tmp, arg));
                tmp
            }

            Node::Add(a, b) => {
                let left = self.gen(a);
                let right = self.gen(b);
                let tmp = self.next_tmp();
                self.emit(format!("float {} = ({} + {}) * 0.5;", tmp, left, right));
                tmp
            }

            Node::Mult(a, b) => {
                let left = self.gen(a);
                let right = self.gen(b);
                let tmp = self.next_tmp();
                self.emit(format!("float {} = {} * {};", tmp, left, right));
                tmp
            }

            Node::Div(a, b) => {
                let left = self.gen(a);
                let right = self.gen(b);
                let tmp = self.next_tmp();
                self.emit(format!("float {tmp} = fabs({right}) > 1e-6 ? ({left} / {right}) : 0.0;"));
                tmp
            }

            Node::MixUnbounded(a, b, c, d) => {
                let a = self.gen(a);
                let b = self.gen(b);
                let c = self.gen(c);
                let d = self.gen(d);
                let tmp = self.next_tmp();
                self.emit(format!("float {} = mixu({}, {}, {}, {});", tmp, a, b, c, d));
                tmp
            }

            _ => unreachable!(),
        }
    }

    pub fn eval_function(&self, name: &str, result_var: &str) -> String {
        let mut out = format!("float {}(float x, float y) {{\n", name);
        for line in &self.lines {
            writeln!(out, "    {}", line).unwrap();
        }
        writeln!(out, "    return {};", result_var).unwrap();
        out.push_str("}\n");
        out
    }
}

pub(crate) fn emit_metal_from_triple(r: &Node, g: &Node, b: &Node) -> String {
    let mut out = String::new();
    out += r#"
#include <metal_stdlib>
using namespace metal;

inline float mixu(float a, float b, float c, float d) {
    return (a * c + b * d) / (a + b + 1e-6);
}
"#;

    let mut ctx_r = CodegenCtx::new();
    let r_final = ctx_r.gen(r);
    out += &ctx_r.eval_function("eval_r", &r_final);
    out += "\n";

    let mut ctx_g = CodegenCtx::new();
    let g_final = ctx_g.gen(g);
    out += &ctx_g.eval_function("eval_g", &g_final);
    out += "\n";

    let mut ctx_b = CodegenCtx::new();
    let b_final = ctx_b.gen(b);
    out += &ctx_b.eval_function("eval_b", &b_final);
    out += "\n";

    out += r#"
kernel void art_gen(texture2d<float, access::write> out [[texture(0)]],
                    uint2 gid [[thread_position_in_grid]]) {
    float2 uv = float2(gid) / float2(out.get_width(), out.get_height());
    float x = uv.x * 2.0 - 1.0;
    float y = uv.y * 2.0 - 1.0;

    float r = eval_r(x, y);
    float g = eval_g(x, y);
    float b = eval_b(x, y);

    out.write(float4((r + 1.0) * 0.5, (g + 1.0) * 0.5, (b + 1.0) * 0.5, 1.0), gid);
}
"#;

    out
}