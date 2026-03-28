use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Module, Linkage};
use randomart_core::node::Node;

macro_rules! define_and_register_math_fns {
    ($builder:ident, [$(($name:ident, $ret:ty, [$($arg:ident : $typ:ty),*], $body:block)),* $(,)?]) => {
        $(
            fn $name($($arg: $typ),*) -> $ret {
                $body
            }
            $builder.symbol(stringify!($name), $name as *const u8);
        )*
    };
}

macro_rules! call_imported_func {
    ($builder:expr, $module:expr, $name:expr, [$($arg:expr),*], [$($ty:expr),*], $ret_ty:expr) => {{
        let mut sig = $module.make_signature();

        $(
            sig.params.push(AbiParam::new($ty));
        )*

        sig.returns.push(AbiParam::new($ret_ty));

        let func = $module
            .declare_function($name, Linkage::Import, &sig)
            .unwrap();

        let local = $module.declare_func_in_func(func, $builder.func);
        let args = &[$($arg),*];
        let call = $builder.ins().call(local, args);
        $builder.inst_results(call)[0]
    }};
}

fn codegen_node(
    builder: &mut FunctionBuilder,
    module: &mut JITModule,
    node: &Node,
    x: Value,
    y: Value,
) -> Value {
    use cranelift::prelude::*;

    match node {
        Node::X => x,
        Node::Y => y,
        Node::Number(val) => builder.ins().f32const(Ieee32::with_float(*val)),

        Node::Add(a, b) => {
            let lhs = codegen_node(builder, module, a, x, y);
            let rhs = codegen_node(builder, module, b, x, y);
            let sum = builder.ins().fadd(lhs, rhs);
            let two = builder.ins().f32const(Ieee32::with_float(2.0));
            builder.ins().fdiv(sum, two)
        }

        Node::Mult(a, b) => {
            let lhs = codegen_node(builder, module, a, x, y);
            let rhs = codegen_node(builder, module, b, x, y);
            builder.ins().fmul(lhs, rhs)
        }

        Node::Sin(inner) => {
            let arg = codegen_node(builder, module, inner, x, y);
            call_imported_func!(builder, module, "my_sin", [arg], [types::F32], types::F32)
        }

        Node::Cos(inner) => {
            let arg = codegen_node(builder, module, inner, x, y);
            call_imported_func!(builder, module, "my_cos", [arg], [types::F32], types::F32)
        }

        Node::Sqrt(inner) => {
            let arg = codegen_node(builder, module, inner, x, y);
            let zero = builder.ins().f32const(Ieee32::with_float(0.0));
            let safe = builder.ins().fmax(arg, zero);
            builder.ins().sqrt(safe)
        }

        Node::Exp(inner) => {
            let arg = codegen_node(builder, module, inner, x, y);
            call_imported_func!(builder, module, "my_exp", [arg], [types::F32], types::F32)
        }

        Node::Div(a, b) => {
            let lhs = codegen_node(builder, module, a, x, y);
            let rhs = codegen_node(builder, module, b, x, y);
            let threshold = builder.ins().f32const(Ieee32::with_float(1e-6));
            let zero = builder.ins().f32const(Ieee32::with_float(0.0));
            let abs_rhs = builder.ins().fabs(rhs);
            let cond = builder.ins().fcmp(FloatCC::GreaterThan, abs_rhs, threshold);
            let quot = builder.ins().fdiv(lhs, rhs);
            builder.ins().select(cond, quot, zero)
        }

        Node::MixUnbounded(a, b, c, d) => {
            let va = codegen_node(builder, module, a, x, y);
            let vb = codegen_node(builder, module, b, x, y);
            let vc = codegen_node(builder, module, c, x, y);
            let vd = codegen_node(builder, module, d, x, y);
            let eps = builder.ins().f32const(Ieee32::with_float(1e-6));
            let rac = builder.ins().fmul(va, vc);
            let rbd = builder.ins().fmul(vb, vd);
            let num = builder.ins().fadd(rac, rbd);
            let ab = builder.ins().fadd(va, vb);
            let denom = builder.ins().fadd(ab, eps);
            builder.ins().fdiv(num, denom)
        }

        Node::Triple(_, _, _) => {
            panic!("Triple node should be handled at the top level, not in scalar codegen")
        }

        Node::Random => {
            panic!("Node::Random must be resolved before JIT")
        }

        Node::Rule(_) => {
            panic!("Node::Rule must be expanded before JIT")
        }
    }
}

fn build_jit_function(ast: &Node) -> Box<dyn Fn(f32, f32) -> f32 + Sync + Send> {
    let mut builder = JITBuilder::new(cranelift_module::default_libcall_names())
        .expect("Failed to create JITBuilder");

    define_and_register_math_fns!(builder, [
        (my_sin, f32, [x: f32], { x.sin() }),
        (my_cos, f32, [x: f32], { x.cos() }),
        (my_exp, f32, [x: f32], { x.exp() }),
    ]);

    let mut module = JITModule::new(builder);

    let mut sig = module.make_signature();
    sig.params.push(AbiParam::new(types::F32));
    sig.params.push(AbiParam::new(types::F32));
    sig.returns.push(AbiParam::new(types::F32));

    let func_id = module
        .declare_function("jit_func", Linkage::Export, &sig)
        .unwrap();

    let mut ctx = module.make_context();
    ctx.func.signature = sig;

    let mut builder_ctx = FunctionBuilderContext::new();
    let mut fb = FunctionBuilder::new(&mut ctx.func, &mut builder_ctx);
    let block = fb.create_block();

    fb.append_block_params_for_function_params(block);
    fb.switch_to_block(block);
    fb.seal_block(block);

    let x = fb.block_params(block)[0];
    let y = fb.block_params(block)[1];
    let result = codegen_node(&mut fb, &mut module, ast, x, y);
    fb.ins().return_(&[result]);
    fb.finalize();

    module.define_function(func_id, &mut ctx).unwrap();
    module.clear_context(&mut ctx);
    let _ = module.finalize_definitions();

    let code = module.get_finalized_function(func_id);
    let fn_ptr = unsafe { std::mem::transmute::<_, fn(f32, f32) -> f32>(code) };
    Box::new(fn_ptr) as Box<dyn Fn(f32, f32) -> f32 + Sync + Send>
}

pub(crate) fn build_jit_function_triple(node: &Node)
-> (
    Box<dyn Fn(f32, f32) -> f32 + Sync + Send>,
    Box<dyn Fn(f32, f32) -> f32 + Sync + Send>,
    Box<dyn Fn(f32, f32) -> f32 + Sync + Send>,
)
{
    let (r, g, b) = match &*node {
        Node::Triple(r, g, b) => (r, g, b),
        _ => panic!("Expected Triple node at top level"),
    };
    let (r_jit_fn, g_jit_fn): (
        Box<dyn Fn(f32, f32) -> f32 + Sync + Send>,
        Box<dyn Fn(f32, f32) -> f32 + Sync + Send>
    ) = rayon::join(
        || build_jit_function(r),
        || build_jit_function(g)
    );
    let b_jit_fn = build_jit_function(b);
    (r_jit_fn, g_jit_fn, b_jit_fn)
}
