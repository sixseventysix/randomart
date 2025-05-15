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
        Node::Number(val) => builder.ins().f32const(*val),

        Node::Add(a, b) => {
            let lhs = codegen_node(builder, module, a, x, y);
            let rhs = codegen_node(builder, module, b, x, y);
            builder.ins().fadd(lhs, rhs)
        }

        Node::Mult(a, b) => {
            let lhs = codegen_node(builder, module, a, x, y);
            let rhs = codegen_node(builder, module, b, x, y);
            builder.ins().fmul(lhs, rhs)
        }

        Node::Div(a, b) => {
            let lhs = codegen_node(builder, module, a, x, y);
            let rhs = codegen_node(builder, module, b, x, y);
            builder.ins().fdiv(lhs, rhs)
        }

        Node::Modulo(a, b) => {
            let lhs = codegen_node(builder, module, a, x, y);
            let rhs = codegen_node(builder, module, b, x, y);

            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));

            let fmodf_func = module
                .declare_function("fmodf", Linkage::Import, &sig)
                .unwrap();
            let local = module.declare_func_in_func(fmodf_func, builder.func);
            let call_inst = builder.ins().call(local, &[lhs, rhs]);
            builder.inst_results(call_inst)[0]
        }

        Node::Sqrt(a) => {
            let val = codegen_node(builder, module, a, x, y);

            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));

            let sqrtf_func = module
                .declare_function("sqrtf", Linkage::Import, &sig)
                .unwrap();
            let local = module.declare_func_in_func(sqrtf_func, builder.func);
            let call_inst = builder.ins().call(local, &[val]);
            builder.inst_results(call_inst)[0]
        }

        Node::Sin(inner) => {
            let arg = codegen_node(builder, module, inner, x, y);
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));

            let sinf_func = module.declare_function("sinf", Linkage::Import, &sig).unwrap();
            let local = module.declare_func_in_func(sinf_func, builder.func);
            let call = builder.ins().call(local, &[arg]);
            builder.inst_results(call)[0]
        }

        Node::Cos(inner) => {
            let arg = codegen_node(builder, module, inner, x, y);
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));

            let cosf_func = module.declare_function("cosf", Linkage::Import, &sig).unwrap();
            let local = module.declare_func_in_func(cosf_func, builder.func);
            let call = builder.ins().call(local, &[arg]);
            builder.inst_results(call)[0]
        }

        Node::Exp(inner) => {
            let arg = codegen_node(builder, module, inner, x, y);
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F32));
            sig.returns.push(AbiParam::new(types::F32));

            let expf_func = module.declare_function("expf", Linkage::Import, &sig).unwrap();
            let local = module.declare_func_in_func(expf_func, builder.func);
            let call = builder.ins().call(local, &[arg]);
            builder.inst_results(call)[0]
        }

        Node::Mix(a, b, c, d) => {
            let a = codegen_node(builder, module, a, x, y);
            let b = codegen_node(builder, module, b, x, y);
            let c = codegen_node(builder, module, c, x, y);
            let d = codegen_node(builder, module, d, x, y);

            let one = builder.ins().f32const(1.0);
            let a1 = builder.ins().fadd(a, one);
            let b1 = builder.ins().fadd(b, one);
            let c1 = builder.ins().fadd(c, one);
            let d1 = builder.ins().fadd(d, one);

            let ac = builder.ins().fmul(a1, c1);
            let bd = builder.ins().fmul(b1, d1);
            let numerator = builder.ins().fadd(ac, bd);

            let ab = builder.ins().fadd(a1, b1);
            let epsilon = builder.ins().f32const(1e-6);
            let denom = builder.ins().fmax(ab, epsilon);

            let div = builder.ins().fdiv(numerator, denom);
            builder.ins().fsub(div, one)
        }

        Node::MixUnbounded(a, b, c, d) => {
            let a = codegen_node(builder, module, a, x, y);
            let b = codegen_node(builder, module, b, x, y);
            let c = codegen_node(builder, module, c, x, y);
            let d = codegen_node(builder, module, d, x, y);

            let ac = builder.ins().fmul(a, c);
            let bd = builder.ins().fmul(b, d);
            let numerator = builder.ins().fadd(ac, bd);

            let ab = builder.ins().fadd(a, b);
            let epsilon = builder.ins().f32const(1e-6);
            let denom = builder.ins().fadd(ab, epsilon);

            builder.ins().fdiv(numerator, denom)
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
    let builder = JITBuilder::new(cranelift_module::default_libcall_names())
        .expect("Failed to create JITBuilder");
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