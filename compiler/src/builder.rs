use crate::utils::{identifier_to_c_str, identifier_to_string, string_to_c_str};
use dishsoap_parser::ast::*;
use llvm_sys::core::{
    LLVMAddFunction, LLVMAppendBasicBlockInContext, LLVMBuildAdd, LLVMBuildAlloca, LLVMBuildCall,
    LLVMBuildLoad, LLVMBuildMul, LLVMBuildRet, LLVMBuildSDiv, LLVMBuildSRem, LLVMBuildStore,
    LLVMBuildSub, LLVMConstInt, LLVMFunctionType, LLVMGetNamedFunction, LLVMGetParams,
    LLVMInt32Type, LLVMPositionBuilderAtEnd, LLVMVoidType,
};
use llvm_sys::prelude::{LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef};
use std::collections::HashMap;
use std::mem::forget;

pub struct Builder<'a> {
    variables: HashMap<String, LLVMValueRef>,
    context: &'a LLVMContextRef,
    module: &'a LLVMModuleRef,
    builder: &'a LLVMBuilderRef,
}

impl<'a> Builder<'a> {
    pub fn new(
        context: &'a LLVMContextRef,
        module: &'a LLVMModuleRef,
        builder: &'a LLVMBuilderRef,
    ) -> Self {
        Builder {
            context,
            module,
            builder,
            variables: HashMap::new(),
        }
    }

    pub fn build(&mut self, node: &'a Node) {
        unsafe {
            match node {
                Node::SourceFile(s) => s.children.iter().for_each(|c| self.build(c)),
                Node::Block(b) => b.statements.iter().for_each(|s| self.build(s)),
                Node::FunctionDeclarationStatement(s) => {
                    let param_count = s.parameters.len();
                    let function_type = LLVMFunctionType(
                        self.visit_type(&*s.return_type),
                        s.parameters
                            .iter()
                            .map(|p| match &**p {
                                Node::VariableLike(v) => self.visit_type(&*v.variable_type),
                                _ => panic!("Compilation error: unexpected node"),
                            })
                            .collect::<Vec<LLVMTypeRef>>()
                            .as_ptr() as *mut _,
                        param_count as u32,
                        0,
                    );
                    let function = LLVMAddFunction(
                        *self.module,
                        identifier_to_c_str(&*s.identifier),
                        function_type,
                    );
                    let block = LLVMAppendBasicBlockInContext(
                        *self.context,
                        function,
                        identifier_to_c_str(&*s.identifier),
                    );
                    LLVMPositionBuilderAtEnd(*self.builder, block);

                    let mut raw_vec: Vec<LLVMValueRef> = Vec::with_capacity(param_count);
                    let ptr = raw_vec.as_mut_ptr();

                    forget(raw_vec);

                    let raw_vec = {
                        LLVMGetParams(function, ptr);
                        Vec::from_raw_parts(ptr, param_count, param_count)
                    };

                    // Global variables and nested function definitions currently aren't supported,
                    // so it's safe to clear the variable "symbol table" when entering a new
                    // function scope.
                    self.variables.clear();
                    raw_vec
                        .iter()
                        .enumerate()
                        .for_each(|(i, p)| match &*s.parameters[i] {
                            Node::VariableLike(v) => {
                                let var = LLVMBuildAlloca(
                                    *self.builder,
                                    self.visit_type(&*v.variable_type),
                                    identifier_to_c_str(&v.identifier),
                                );
                                LLVMBuildStore(*self.builder, *p, var);
                                self.variables
                                    .insert(identifier_to_string(&*v.identifier), var);
                            }
                            _ => panic!("Compilation error: expected parameter"),
                        });
                    match &*s.body {
                        Node::Block(b) => {
                            if b.statements.len() > 1 {
                                &b.statements[..b.statements.len() - 1]
                                    .iter()
                                    .for_each(|s| self.build(s));
                            }
                            if b.statements.len() >= 1 {
                                self.build(b.statements.last().unwrap());
                            }
                        }
                        _ => panic!("Compilation error: unexpected node"),
                    }
                }
                Node::VariableDeclarationStatement(s) => {
                    match &*s.variable {
                        Node::VariableLike(v) => {
                            let var = LLVMBuildAlloca(
                                *self.builder,
                                self.visit_type(&*v.variable_type),
                                identifier_to_c_str(&*v.identifier),
                            );
                            LLVMBuildStore(
                                *self.builder,
                                self.visit((*v.initial_value).as_ref().unwrap()),
                                var,
                            );
                            self.variables
                                .insert(identifier_to_string(&*v.identifier), var);
                        }
                        _ => panic!("Compilation error: unexpected node"),
                    };
                }
                Node::BinaryExpression(_) => {
                    self.visit(node);
                }
                Node::ReturnStatement(ret_s) => match ret_s {
                    ReturnStatement { expression } => {
                        LLVMBuildRet(*self.builder, self.visit(&*expression));
                    }
                },
                _ => (),
            }
        }
    }

    pub fn visit(&self, node: &'a Node) -> LLVMValueRef {
        unsafe {
            match node {
                Node::IntegerLiteral { value } => LLVMConstInt(LLVMInt32Type(), *value as u64, 1),
                Node::VariableReference(v) => match &*v.identifier {
                    Node::Identifier(i) => LLVMBuildLoad(
                        *self.builder,
                        *self.variables.get(&i.name).unwrap(),
                        string_to_c_str(&"ld_tmp".to_owned()),
                    ),
                    _ => panic!("Compilation error"),
                },
                Node::FunctionCall(f) => {
                    let function =
                        LLVMGetNamedFunction(*self.module, identifier_to_c_str(&*f.identifier));

                    LLVMBuildCall(
                        *self.builder,
                        function,
                        f.arguments
                            .iter()
                            .map(|a| self.visit(a))
                            .collect::<Vec<LLVMValueRef>>()
                            .as_ptr() as *mut _,
                        f.arguments.len() as u32,
                        string_to_c_str(&"call_tmp".to_owned()),
                    )
                }
                Node::BinaryExpression(e) => match &*e.operator {
                    Node::InfixOperator(op) => match op {
                        InfixOperator::Plus => LLVMBuildAdd(
                            *self.builder,
                            self.visit(&*e.left),
                            self.visit(&*e.right),
                            string_to_c_str(&"add_tmp".to_owned()),
                        ),
                        InfixOperator::Minus => LLVMBuildSub(
                            *self.builder,
                            self.visit(&*e.left),
                            self.visit(&*e.right),
                            string_to_c_str(&"sub_tmp".to_owned()),
                        ),
                        InfixOperator::Times => LLVMBuildMul(
                            *self.builder,
                            self.visit(&*e.left),
                            self.visit(&*e.right),
                            string_to_c_str(&"mul_tmp".to_owned()),
                        ),
                        InfixOperator::Divide => LLVMBuildSDiv(
                            *self.builder,
                            self.visit(&*e.left),
                            self.visit(&*e.right),
                            string_to_c_str(&"sdiv_tmp".to_owned()),
                        ),
                        InfixOperator::Modulo => LLVMBuildSRem(
                            *self.builder,
                            self.visit(&*e.left),
                            self.visit(&*e.right),
                            string_to_c_str(&"srem_tmp".to_owned()),
                        ),
                        _ => panic!("Compilation error: unexpected node"),
                    },
                    _ => panic!("Compilation error: unexpected node"),
                },
                _ => panic!("Compilation error: unexpected node"),
            }
        }
    }

    fn visit_type(&self, node: &'a Node) -> LLVMTypeRef {
        unsafe {
            match node {
                Node::TypeLiteral(i) => match i {
                    TypeLiteral::Int => LLVMInt32Type(),
                    TypeLiteral::Void => LLVMVoidType(),
                },
                _ => panic!("Compilation error: unexpected node"),
            }
        }
    }
}
