use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Module, Linkage};
use crate::node::Node;

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

            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));

            let func = module.declare_function("my_add", Linkage::Import, &sig).unwrap();
            let local = module.declare_func_in_func(func, builder.func);
            let call = builder.ins().call(local, &[lhs, rhs]);
            builder.inst_results(call)[0]
        }

        Node::Mult(a, b) => {
            let lhs = codegen_node(builder, module, a, x, y);
            let rhs = codegen_node(builder, module, b, x, y);

            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));

            let func = module.declare_function("my_mul", Linkage::Import, &sig).unwrap();
            let local = module.declare_func_in_func(func, builder.func);
            let call = builder.ins().call(local, &[lhs, rhs]);
            builder.inst_results(call)[0]
        }

        Node::Sin(inner) => {
            let arg = codegen_node(builder, module, inner, x, y);

            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));

            let func = module.declare_function("my_sin", Linkage::Import, &sig).unwrap();
            let local = module.declare_func_in_func(func, builder.func);
            let call = builder.ins().call(local, &[arg]);
            builder.inst_results(call)[0]
        }

        Node::Cos(inner) => {
            let arg = codegen_node(builder, module, inner, x, y);

            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));

            let func = module.declare_function("my_cos", Linkage::Import, &sig).unwrap();
            let local = module.declare_func_in_func(func, builder.func);
            let call = builder.ins().call(local, &[arg]);
            builder.inst_results(call)[0]
        }

        Node::Sqrt(inner) => {
            let arg = codegen_node(builder, module, inner, x, y);

            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));

            let func = module.declare_function("my_sqrt", Linkage::Import, &sig).unwrap();
            let local = module.declare_func_in_func(func, builder.func);
            let call = builder.ins().call(local, &[arg]);
            builder.inst_results(call)[0]
        }

        Node::Exp(inner) => {
            let arg = codegen_node(builder, module, inner, x, y);

            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));

            let func = module.declare_function("my_exp", Linkage::Import, &sig).unwrap();
            let local = module.declare_func_in_func(func, builder.func);
            let call = builder.ins().call(local, &[arg]);
            builder.inst_results(call)[0]
        }

        Node::Div(a, b) => {
            let lhs = codegen_node(builder, module, a, x, y);
            let rhs = codegen_node(builder, module, b, x, y);

            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));

            let func = module.declare_function("my_div", Linkage::Import, &sig).unwrap();
            let local = module.declare_func_in_func(func, builder.func);
            let call = builder.ins().call(local, &[lhs, rhs]);
            builder.inst_results(call)[0]
        }

        Node::Modulo(a, b) => {
            let lhs = codegen_node(builder, module, a, x, y);
            let rhs = codegen_node(builder, module, b, x, y);

            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));

            let func = module.declare_function("my_mod", Linkage::Import, &sig).unwrap();
            let local = module.declare_func_in_func(func, builder.func);
            let call = builder.ins().call(local, &[lhs, rhs]);
            builder.inst_results(call)[0]
        }

        Node::Mix(a, b, c, d) => {
            let va = codegen_node(builder, module, a, x, y);
            let vb = codegen_node(builder, module, b, x, y);
            let vc = codegen_node(builder, module, c, x, y);
            let vd = codegen_node(builder, module, d, x, y);

            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.params.push(AbiParam::new(types::F32));
            sig.params.push(AbiParam::new(types::F32));
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));

            let func = module.declare_function("my_mix", Linkage::Import, &sig).unwrap();
            let local = module.declare_func_in_func(func, builder.func);
            let call = builder.ins().call(local, &[va, vb, vc, vd]);
            builder.inst_results(call)[0]
        }

        Node::MixUnbounded(a, b, c, d) => {
            let va = codegen_node(builder, module, a, x, y);
            let vb = codegen_node(builder, module, b, x, y);
            let vc = codegen_node(builder, module, c, x, y);
            let vd = codegen_node(builder, module, d, x, y);

            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.params.push(AbiParam::new(types::F32));
            sig.params.push(AbiParam::new(types::F32));
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));

            let func = module.declare_function("my_mixu", Linkage::Import, &sig).unwrap();
            let local = module.declare_func_in_func(func, builder.func);
            let call = builder.ins().call(local, &[va, vb, vc, vd]);
            builder.inst_results(call)[0]
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

pub fn build_jit_function(ast: &Node) -> Box<dyn Fn(f32, f32) -> f32 + Sync> {
    let mut builder = JITBuilder::new(cranelift_module::default_libcall_names())
        .expect("Failed to create JITBuilder");

    extern "C" fn my_sin(x: f32) -> f32 { x.sin() }
    extern "C" fn my_cos(x: f32) -> f32 { x.cos() }
    extern "C" fn my_sqrt(x: f32) -> f32 { x.sqrt().max(0.0) }
    extern "C" fn my_exp(x: f32) -> f32 { x.exp() }

    extern "C" fn my_add(a: f32, b: f32) -> f32 { (a + b) / 2.0 }
    extern "C" fn my_mul(a: f32, b: f32) -> f32 { a * b }
    extern "C" fn my_div(a: f32, b: f32) -> f32 {
        if b.abs() > 1e-6 { a / b } else { 0.0 }
    }
    extern "C" fn my_mod(a: f32, b: f32) -> f32 {
        if b.abs() > 1e-6 { a % b } else { 0.0 }
    }
    extern "C" fn my_mix(a: f32, b: f32, c: f32, d: f32) -> f32 {
        let a = a + 1.0;
        let b = b + 1.0;
        let c = c + 1.0;
        let d = d + 1.0;
        let numerator = a * c + b * d;
        let denominator = (a + b).max(1e-6);
        (numerator / denominator) - 1.0
    }
    extern "C" fn my_mixu(a: f32, b: f32, c: f32, d: f32) -> f32 {
        (a * c + b * d) / (a + b + 1e-6)
    }

    builder.symbol("my_sin", my_sin as *const u8);
    builder.symbol("my_cos", my_cos as *const u8);
    builder.symbol("my_sqrt", my_sqrt as *const u8);
    builder.symbol("my_exp", my_exp as *const u8);

    builder.symbol("my_add", my_add as *const u8);
    builder.symbol("my_mul", my_mul as *const u8);
    builder.symbol("my_div", my_div as *const u8);
    builder.symbol("my_mod", my_mod as *const u8);

    builder.symbol("my_mix", my_mix as *const u8);
    builder.symbol("my_mixu", my_mixu as *const u8);

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

    // println!("{}", ctx.func.display());
    module.define_function(func_id, &mut ctx).unwrap();
    module.clear_context(&mut ctx);
    let _ = module.finalize_definitions();

    let code = module.get_finalized_function(func_id);
    let fn_ptr = unsafe { std::mem::transmute::<_, fn(f32, f32) -> f32>(code) };
    Box::new(fn_ptr) as Box<dyn Fn(f32, f32) -> f32 + Sync>
}