use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::values::BasicValueEnum;
use crate::parser::{Expr, Function};

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        CodeGen { context, builder, module }
    }

    pub fn codegen_function(&self, function: &Function) {
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
        let function = self.module.add_function(&function.name, fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        let mut vars = std::collections::HashMap::new();
        for (i, param) in function.get_params().iter().enumerate() {
            let param_name = &function.params[i];
            vars.insert(param_name.clone(), param.clone());
        }

        let body = self.codegen_expr(&function.body, &vars);
        self.builder.build_return(Some(&body));
    }

    fn codegen_expr(&self, expr: &Expr, vars: &std::collections::HashMap<String, BasicValueEnum<'ctx>>) -> BasicValueEnum<'ctx> {
        match expr {
            Expr::Number(n) => self.context.i32_type().const_int(*n as u64, false).into(),
            Expr::Identifier(name) => *vars.get(name).unwrap(),
            Expr::Binary(lhs, op, rhs) => {
                let lhs = self.codegen_expr(lhs, vars).into_int_value();
                let rhs = self.codegen_expr(rhs, vars).into_int_value();
                match op {
                    _ => self.builder.build_int_add(lhs, rhs, "tmpadd").into(),
                }
            }
        }
    }
}
