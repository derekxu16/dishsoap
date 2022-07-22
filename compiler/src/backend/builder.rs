use crate::utils::{identifier_to_c_string, identifier_to_string, string_to_c_string};
use crate::visitor::{PreOrderVisitor, PreOrderVisitorResponse};
use dishsoap_parser::ast::*;
use itertools::Itertools;
use llvm_sys::core::*;
pub use llvm_sys::prelude::*;
use llvm_sys::LLVMIntPredicate;
use std::collections::HashMap;
use std::mem::forget;
use std::ops::Deref;
use std::rc::Rc;

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum LLVMRefType {
//     LLVMBool(LLVMBool),
//     LLVMMemoryBufferRef(LLVMMemoryBufferRef),
//     LLVMContextRef(LLVMContextRef),
//     LLVMModuleRef(LLVMModuleRef),
//     LLVMTypeRef(LLVMTypeRef),
//     LLVMValueRef(LLVMValueRef),
//     LLVMBasicBlockRef(LLVMBasicBlockRef),
//     LLVMMetadataRef(LLVMMetadataRef),
//     LLVMNamedMDNodeRef(LLVMNamedMDNodeRef),
//     LLVMValueMetadataEntry(LLVMValueMetadataEntry),
//     LLVMBuilderRef(LLVMBuilderRef),
//     LLVMDIBuilderRef(LLVMDIBuilderRef),
//     LLVMModuleProviderRef(LLVMModuleProviderRef),
//     LLVMPassManagerRef(LLVMPassManagerRef),
//     LLVMPassRegistryRef(LLVMPassRegistryRef),
//     LLVMUseRef(LLVMUseRef),
//     LLVMDiagnosticInfoRef(LLVMDiagnosticInfoRef),
//     LLVMComdatRef(LLVMComdatRef),
//     LLVMModuleFlagEntry(LLVMModuleFlagEntry),
//     LLVMJITEventListenerRef(LLVMJITEventListenerRef),
//     LLVMAttributeRef(LLVMAttributeRef),
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct LoweredNodeCommonFields {
//     llvm_ref: LLVMRefType,
// }

// impl LoweredNodeCommonFields {
//     fn new(llvm_ref: LLVMRefType) -> LoweredNodeCommonFields {
//         LoweredNodeCommonFields { llvm_ref }
//     }
// }

// pub trait Lowered {
//     fn get_llvm_ref(&self) -> LLVMRefType;
// }

// impl Lowered for Expression<LoweredNodeCommonFields> {
//     fn get_llvm_ref(&self) -> LLVMRefType {
//         match self {
//             Expression::IntegerLiteral(i) => i.common_fields.llvm_ref.clone(),
//             Expression::VariableReference(r) => r.common_fields.llvm_ref.clone(),
//             Expression::FunctionCall(c) => unimplemented!(),
//             Expression::PrefixExpression(e) => unimplemented!(),
//             Expression::BinaryExpression(e) => e.common_fields.llvm_ref.clone(),
//             Expression::IfExpression(e) => unimplemented!(),
//         }
//     }
// }

// struct ExpressionSeqeunce {
//     instructions: Vec<>,
//     expression:
// }

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

    pub fn lower_type_literal(&mut self, r#type: &TypeLiteral) -> LLVMTypeRef {
        unsafe {
            match r#type {
                TypeLiteral::UnitType => LLVMVoidType(),
                TypeLiteral::BoolType => LLVMInt1Type(),
                TypeLiteral::I32Type => LLVMInt32Type(),
            }
        }
    }

    pub fn lower_record_type(&mut self, r#type: &RecordType) -> LLVMTypeRef {
        let element_count = r#type.fields.keys().len();
        let sorted_keys = r#type.fields.keys().sorted().collect::<Vec<&String>>();
        let mut element_types = sorted_keys
            .iter()
            .map(|k| self.lower_type(r#type.fields.get(&**k).unwrap()))
            .collect::<Vec<LLVMTypeRef>>();

        unsafe {
            LLVMPointerType(
                LLVMStructType(
                    element_types.as_mut_ptr(),
                    element_count as u32,
                    false.into(),
                ),
                0,
            )
        }
    }

    pub fn lower_function_type(&mut self, r#_type: &FunctionType) -> LLVMTypeRef {
        unimplemented!()
    }

    pub fn lower_type(&mut self, r#type: &Type) -> LLVMTypeRef {
        match r#type {
            Type::TypeLiteral(t) => self.lower_type_literal(t),
            Type::RecordType(t) => self.lower_record_type(&**t),
            Type::FunctionType(t) => self.lower_function_type(&**t),
        }
    }

    pub fn lower_boolean_literal(
        &self,
        boolean_literal: &BooleanLiteral<TypedNodeCommonFields>,
    ) -> LLVMValueRef {
        match boolean_literal {
            BooleanLiteral {
                common_fields: _,
                value,
            } => unsafe { LLVMConstInt(LLVMInt1Type(), *value as u64, false.into()) },
        }
    }

    pub fn lower_integer_literal(
        &self,
        integer_literal: &IntegerLiteral<TypedNodeCommonFields>,
    ) -> LLVMValueRef {
        match integer_literal {
            IntegerLiteral {
                common_fields: _,
                value,
            } => unsafe { LLVMConstInt(LLVMInt32Type(), *value as u64, true.into()) },
        }
    }

    pub fn lower_record_literal(
        &mut self,
        record_literal: &RecordLiteral<TypedNodeCommonFields>,
    ) -> LLVMValueRef {
        unsafe {
            let record_pointer = LLVMBuildMalloc(
                *self.builder,
                // TODO(derekxu16): Record types get lowered to pointer types in every case except
                // this one. Look for a more elegant way of handling this.
                LLVMGetElementType(self.lower_type(&record_literal.common_fields.r#type)),
                string_to_c_string("record_literal_malloc_temp".to_owned()).as_ptr(),
            );
            for (index, key) in record_literal.fields.keys().sorted().enumerate() {
                LLVMBuildStore(
                    *self.builder,
                    self.lower_expression(record_literal.fields.get(&*key).unwrap()),
                    LLVMBuildStructGEP(
                        *self.builder,
                        record_pointer,
                        index as u32,
                        string_to_c_string("record_literal_field_pointer_temp".to_string())
                            .as_ptr(),
                    ),
                );
            }

            record_pointer
        }
    }

    pub fn lower_variable_reference(
        &mut self,
        variable_reference: &VariableReference<TypedNodeCommonFields>,
    ) -> LLVMValueRef {
        match variable_reference {
            VariableReference {
                common_fields,
                identifier,
            } => unsafe {
                LLVMBuildLoad2(
                    *self.builder,
                    self.lower_type(&common_fields.r#type),
                    *self.variables.get(&identifier.name).unwrap(),
                    identifier_to_c_string(identifier).as_ptr(),
                )
            },
        }
    }

    pub fn lower_function_call(
        &mut self,
        function_call: &FunctionCall<TypedNodeCommonFields>,
    ) -> LLVMValueRef {
        match function_call {
            FunctionCall {
                common_fields: _,
                identifier,
                arguments,
            } => unsafe {
                let function =
                    LLVMGetNamedFunction(*self.module, identifier_to_c_string(identifier).as_ptr());

                LLVMBuildCall(
                    *self.builder,
                    function,
                    arguments
                        .iter()
                        .map(|a| self.lower_expression(a))
                        .collect::<Vec<LLVMValueRef>>()
                        .as_mut_ptr(),
                    arguments.len() as u32,
                    string_to_c_string("call_temp".to_owned()).as_ptr(),
                )
            },
        }
    }

    pub fn lower_if_expression(
        &mut self,
        if_expression: &IfExpression<TypedNodeCommonFields>,
    ) -> LLVMValueRef {
        match if_expression {
            IfExpression {
                common_fields,
                condition,
                then_block,
                else_block,
            } => unsafe {
                let function = LLVMGetBasicBlockParent(LLVMGetInsertBlock(*self.builder));

                // The value that this if-expression evaluates to.
                let result_value = LLVMBuildAlloca(
                    *self.builder,
                    self.lower_type(&common_fields.r#type),
                    string_to_c_string("if_result_temp".to_owned()).as_ptr(),
                );

                let condition = self.lower_expression(&condition);
                let then_bb = LLVMCreateBasicBlockInContext(
                    *self.context,
                    string_to_c_string("then_block".to_owned()).as_ptr(),
                );
                let else_bb = LLVMCreateBasicBlockInContext(
                    *self.context,
                    string_to_c_string("else_block".to_owned()).as_ptr(),
                );
                let merge_bb = LLVMCreateBasicBlockInContext(
                    *self.context,
                    string_to_c_string("merge_block".to_owned()).as_ptr(),
                );
                LLVMBuildCondBr(*self.builder, condition, then_bb, else_bb);

                LLVMAppendExistingBasicBlock(function, then_bb);
                LLVMPositionBuilderAtEnd(*self.builder, then_bb);
                self.visit(&Node::Block(then_block.clone()));
                LLVMBuildStore(
                    *self.builder,
                    // TODO(derekxu16): This is unsafe.
                    self.lower_expression(then_block.final_expression.as_ref().unwrap()),
                    result_value,
                );
                LLVMBuildBr(*self.builder, merge_bb);

                LLVMAppendExistingBasicBlock(function, else_bb);
                LLVMPositionBuilderAtEnd(*self.builder, else_bb);
                self.visit(&Node::Block(else_block.clone()));
                LLVMBuildStore(
                    *self.builder,
                    // TODO(derekxu16): This is unsafe.
                    self.lower_expression(else_block.final_expression.as_ref().unwrap()),
                    result_value,
                );
                LLVMBuildBr(*self.builder, merge_bb);

                LLVMAppendExistingBasicBlock(function, merge_bb);
                LLVMPositionBuilderAtEnd(*self.builder, merge_bb);

                LLVMBuildLoad(
                    *self.builder,
                    result_value,
                    string_to_c_string("load_temp".to_owned()).as_ptr(),
                )
            },
        }
    }

    pub fn lower_prefix_expression(
        &mut self,
        prefix_expression: &PrefixExpression<TypedNodeCommonFields>,
    ) -> LLVMValueRef {
        match prefix_expression {
            PrefixExpression {
                common_fields: _,
                operator,
                operand,
            } => unsafe {
                match operator {
                    PrefixOperator::Minus => LLVMBuildSub(
                        *self.builder,
                        LLVMConstInt(self.lower_type(operand.get_type()), 0, false.into()),
                        self.lower_expression(operand),
                        string_to_c_string("sub_temp".to_owned()).as_ptr(),
                    ),
                    PrefixOperator::Bang => LLVMBuildXor(
                        *self.builder,
                        LLVMConstInt(LLVMInt1Type(), 1, false.into()),
                        self.lower_expression(operand),
                        string_to_c_string("xor_temp".to_owned()).as_ptr(),
                    ),
                }
            },
        }
    }

    pub fn lower_binary_expression(
        &mut self,
        binary_expression: &BinaryExpression<TypedNodeCommonFields>,
    ) -> LLVMValueRef {
        match binary_expression {
            BinaryExpression {
                common_fields: _,
                left,
                operator,
                right,
            } => unsafe {
                match operator {
                    InfixOperator::Plus => LLVMBuildAdd(
                        *self.builder,
                        self.lower_expression(left),
                        self.lower_expression(right),
                        string_to_c_string("add_temp".to_owned()).as_ptr(),
                    ),
                    InfixOperator::Minus => LLVMBuildSub(
                        *self.builder,
                        self.lower_expression(left),
                        self.lower_expression(right),
                        string_to_c_string("sub_temp".to_owned()).as_ptr(),
                    ),
                    InfixOperator::Times => LLVMBuildMul(
                        *self.builder,
                        self.lower_expression(left),
                        self.lower_expression(right),
                        string_to_c_string("mul_temp".to_owned()).as_ptr(),
                    ),
                    InfixOperator::Divide => LLVMBuildSDiv(
                        *self.builder,
                        self.lower_expression(left),
                        self.lower_expression(right),
                        string_to_c_string("sdiv_temp".to_owned()).as_ptr(),
                    ),
                    InfixOperator::Modulo => LLVMBuildSRem(
                        *self.builder,
                        self.lower_expression(left),
                        self.lower_expression(right),
                        string_to_c_string("srem_temp".to_owned()).as_ptr(),
                    ),
                    InfixOperator::DoubleEquals => LLVMBuildICmp(
                        *self.builder,
                        LLVMIntPredicate::LLVMIntEQ,
                        self.lower_expression(left),
                        self.lower_expression(right),
                        string_to_c_string("eq_temp".to_owned()).as_ptr(),
                    ),
                    InfixOperator::LessThan => LLVMBuildICmp(
                        *self.builder,
                        LLVMIntPredicate::LLVMIntSLT,
                        self.lower_expression(left),
                        self.lower_expression(right),
                        string_to_c_string("slt_tmp".to_owned()).as_ptr(),
                    ),
                    InfixOperator::LessThanEquals => LLVMBuildICmp(
                        *self.builder,
                        LLVMIntPredicate::LLVMIntSLE,
                        self.lower_expression(left),
                        self.lower_expression(right),
                        string_to_c_string("sle_tmp".to_owned()).as_ptr(),
                    ),
                    InfixOperator::GreaterThan => LLVMBuildICmp(
                        *self.builder,
                        LLVMIntPredicate::LLVMIntSGT,
                        self.lower_expression(left),
                        self.lower_expression(right),
                        string_to_c_string("sgt_tmp".to_owned()).as_ptr(),
                    ),
                    InfixOperator::GreaterThanEquals => LLVMBuildICmp(
                        *self.builder,
                        LLVMIntPredicate::LLVMIntSGE,
                        self.lower_expression(left),
                        self.lower_expression(right),
                        string_to_c_string("sge_tmp".to_owned()).as_ptr(),
                    ),
                    _ => {
                        unimplemented!()
                    }
                }
            },
        }
    }

    pub fn lower_field_access(
        &mut self,
        field_access: &FieldAccess<TypedNodeCommonFields>,
    ) -> LLVMValueRef {
        let target_type = match field_access.target.get_type() {
            Type::RecordType(t) => t,
            _ => unreachable!(),
        };

        unsafe {
            let element_pointer = LLVMBuildStructGEP(
                *self.builder,
                self.lower_expression(&field_access.target),
                target_type
                    .fields
                    .keys()
                    .sorted()
                    .position(|k| *k == field_access.field_name)
                    .unwrap() as u32,
                string_to_c_string("field_access_pointer_temp".to_owned()).as_ptr(),
            );
            LLVMBuildLoad(
                *self.builder,
                element_pointer,
                string_to_c_string("field_access_temp".to_owned()).as_ptr(),
            )
        }
    }

    pub fn lower_expression(
        &mut self,
        expression: &Expression<TypedNodeCommonFields>,
    ) -> LLVMValueRef {
        match expression {
            Expression::UnitLiteral(_u) => unimplemented!(),
            Expression::BooleanLiteral(b) => self.lower_boolean_literal(b),
            Expression::IntegerLiteral(i) => self.lower_integer_literal(i),
            Expression::RecordLiteral(r) => self.lower_record_literal(r),
            Expression::VariableReference(r) => self.lower_variable_reference(r),
            Expression::FunctionCall(c) => self.lower_function_call(c),
            Expression::IfExpression(e) => self.lower_if_expression(e),
            Expression::PrefixExpression(e) => self.lower_prefix_expression(e),
            Expression::BinaryExpression(e) => self.lower_binary_expression(e),
            Expression::FieldAccess(a) => self.lower_field_access(a),
        }
    }
}

impl<'a> PreOrderVisitor<TypedNodeCommonFields> for Builder<'a> {
    fn process_return_statement(
        &mut self,
        return_statement: &ReturnStatement<TypedNodeCommonFields>,
    ) -> PreOrderVisitorResponse {
        match return_statement {
            ReturnStatement { expression } => {
                let lowered_expression = self.lower_expression(expression);
                unsafe {
                    LLVMBuildRet(*self.builder, lowered_expression);
                }
                *PreOrderVisitorResponse::new(true)
            }
        }
    }

    fn process_function_declaration(
        &mut self,
        function_declaration: &FunctionDeclaration<TypedNodeCommonFields>,
    ) -> PreOrderVisitorResponse {
        match function_declaration {
            FunctionDeclaration {
                common_fields: _,
                identifier,
                return_type,
                parameters,
                body,
            } => unsafe {
                let parameter_count = parameters.len();
                let function_type = LLVMFunctionType(
                    self.lower_type(return_type),
                    function_declaration
                        .parameters
                        .iter()
                        .map(|p| self.lower_type(&p.common_fields.r#type))
                        .collect::<Vec<LLVMTypeRef>>()
                        .as_mut_ptr(),
                    parameter_count as u32,
                    0,
                );
                let function = LLVMAddFunction(
                    *self.module,
                    identifier_to_c_string(identifier).as_ptr(),
                    function_type,
                );
                let block = LLVMAppendBasicBlockInContext(
                    *self.context,
                    function,
                    identifier_to_c_string(identifier).as_ptr(),
                );
                LLVMPositionBuilderAtEnd(*self.builder, block);

                let mut raw_vec: Vec<LLVMValueRef> = Vec::with_capacity(parameter_count);
                let raw_vec_as_ptr = raw_vec.as_mut_ptr();
                forget(raw_vec);

                let params = {
                    LLVMGetParams(function, raw_vec_as_ptr);
                    Vec::from_raw_parts(raw_vec_as_ptr, parameter_count, parameter_count)
                };

                // Global variables and nested function definitions currently aren't supported,
                // so it's safe to clear the variable "symbol table" when entering a new
                // function scope.
                self.variables.clear();
                params.iter().enumerate().for_each(|(i, llvm_p)| {
                    let p = &function_declaration.parameters[i];
                    let identifier = &p.variable_declarator.identifier;
                    let var = LLVMBuildAlloca(
                        *self.builder,
                        self.lower_type(&p.common_fields.r#type),
                        identifier_to_c_string(identifier).as_ptr(),
                    );
                    LLVMBuildStore(*self.builder, *llvm_p, var);
                    self.variables.insert(identifier_to_string(identifier), var);
                });

                body.statements.iter().for_each(|s| {
                    self.visit(&Node::Statement(s.clone()));
                });
                match &body.final_expression {
                    Some(e) => {
                        self.visit(&Node::Statement(Statement::ReturnStatement(Rc::new(
                            ReturnStatement::new(e.deref().clone()),
                        ))));
                    }
                    None => (),
                };
            },
        }

        *PreOrderVisitorResponse::new(true)
    }

    fn process_variable_declaration(
        &mut self,
        variable_declaration: &VariableDeclaration<TypedNodeCommonFields>,
    ) -> PreOrderVisitorResponse {
        match variable_declaration {
            VariableDeclaration {
                common_fields: _,
                variable_declarator,
                initial_value,
            } => {
                let identifier = &variable_declarator.identifier;
                unsafe {
                    let var = LLVMBuildAlloca(
                        *self.builder,
                        self.lower_type(&variable_declaration.common_fields.r#type),
                        identifier_to_c_string(identifier).as_ptr(),
                    );
                    LLVMBuildStore(*self.builder, self.lower_expression(&initial_value), var);
                    self.variables.insert(identifier_to_string(identifier), var);
                }

                *PreOrderVisitorResponse::new(true)
            }
        }
    }
}
