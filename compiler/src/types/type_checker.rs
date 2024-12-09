use std::iter::FromIterator;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use crate::visitor::PostOrderVisitor;
use dishsoap_parser::ast::*;

use super::{
    build_environment_from_top_level_declarations,
    populate_type_environment_from_top_level_declarations, EnvironmentStack, TypeEnvironment,
};

pub struct TypeChecker {
    environment_stack: EnvironmentStack,
    type_environment: TypeEnvironment,
}

impl TypeChecker {
    pub fn new(untyped_ast: &Node<UntypedNodeCommonFields>) -> TypeChecker {
        let class_name_to_declaration: Rc<RefCell<HashMap<String, ClassDeclaration>>> =
            match untyped_ast {
                Node::SourceFile(source_file) => Rc::new(RefCell::new(HashMap::from_iter(
                    source_file
                        .type_declarations
                        .iter()
                        .map(|d| (d.identifier.name.clone(), d.clone())),
                ))),
                _ => unreachable!(),
            };
        let type_reference_to_record_type_converters = Rc::new(RefCell::new(HashMap::new()));
        populate_type_environment_from_top_level_declarations(
            type_reference_to_record_type_converters.clone(),
            class_name_to_declaration,
            &untyped_ast,
        );
        TypeChecker {
            environment_stack: EnvironmentStack::new(
                build_environment_from_top_level_declarations(untyped_ast),
            ),
            type_environment: TypeEnvironment {
                type_reference_to_record_type_converters: type_reference_to_record_type_converters
                    .take(),
            },
        }
    }
}

impl PostOrderVisitor<UntypedNodeCommonFields, TypedNodeCommonFields> for TypeChecker {
    fn process_unit_literal(&mut self) -> UnitLiteral<TypedNodeCommonFields> {
        UnitLiteral::<TypedNodeCommonFields>::new()
    }

    fn process_boolean_literal(&mut self, value: &bool) -> BooleanLiteral<TypedNodeCommonFields> {
        BooleanLiteral::<TypedNodeCommonFields>::new(*value)
    }

    fn process_integer_literal(&mut self, value: &i32) -> IntegerLiteral<TypedNodeCommonFields> {
        IntegerLiteral::<TypedNodeCommonFields>::new(*value)
    }

    fn process_object_literal(
        &mut self,
        class: &TypeReference,
        fields: &HashMap<String, Expression<TypedNodeCommonFields>>,
    ) -> ObjectLiteral<TypedNodeCommonFields> {
        let r#type = match self
            .type_environment
            .type_reference_to_record_type_converters
            .get(&class.identifier.name)
        {
            Some(converter) => (**converter)(class),
            None => panic!("Compilation error"),
        };
        ObjectLiteral::<TypedNodeCommonFields>::new(*r#type, class.clone(), fields.clone())
    }

    fn process_variable_reference(
        &mut self,
        identifier: &Identifier,
    ) -> VariableReference<TypedNodeCommonFields> {
        VariableReference::<TypedNodeCommonFields>::new(
            self.environment_stack
                .top()
                .get(&identifier.name)
                .unwrap()
                .clone(),
            identifier.clone(),
        )
    }

    fn process_function_call(
        &mut self,
        identifier: &Identifier,
        arguments: &Vec<Expression<TypedNodeCommonFields>>,
    ) -> FunctionCall<TypedNodeCommonFields> {
        let signature = match self.environment_stack.top().get(&identifier.name).unwrap() {
            Type::FunctionType(t) => t,
            _ => unreachable!(),
        };
        if !(signature.parameter_types.len() == arguments.len()
            && Iterator::zip(signature.parameter_types.iter(), arguments.iter())
                .all(|(p, a)| p == a.get_type()))
        {
            panic!("Compilation error: incompatible types")
        }
        FunctionCall::<TypedNodeCommonFields>::new(
            signature.return_type.clone(),
            identifier.clone(),
            arguments.clone(),
        )
    }

    fn process_if_expression(
        &mut self,
        condition: &Expression<TypedNodeCommonFields>,
        then_block: &Rc<Block<TypedNodeCommonFields>>,
        else_block: &Rc<Block<TypedNodeCommonFields>>,
    ) -> IfExpression<TypedNodeCommonFields> {
        IfExpression::<TypedNodeCommonFields>::new(
            Type::I32Type,
            condition.clone(),
            then_block.clone(),
            else_block.clone(),
        )
    }

    fn process_prefix_expression(
        &mut self,
        operator: &PrefixOperator,
        operand: &Expression<TypedNodeCommonFields>,
    ) -> PrefixExpression<TypedNodeCommonFields> {
        match operator {
            PrefixOperator::Bang => match operand.get_type() {
                Type::BoolType => (),
                _ => panic!("Compilation error: incompatible types"),
            },
            PrefixOperator::Minus => match operand.get_type() {
                Type::I32Type => (),
                _ => panic!("Compilation error: incompatible types"),
            },
        };

        PrefixExpression::<TypedNodeCommonFields>::new(
            operand.get_type().clone(),
            operator.clone(),
            operand.clone(),
        )
    }

    fn process_binary_expression(
        &mut self,
        left: &Expression<TypedNodeCommonFields>,
        operator: &InfixOperator,
        right: &Expression<TypedNodeCommonFields>,
    ) -> BinaryExpression<TypedNodeCommonFields> {
        if left.get_type() != right.get_type() {
            panic!("Compilation error")
        }
        BinaryExpression::<TypedNodeCommonFields>::new(
            left.get_type().clone(),
            left.clone(),
            operator.clone(),
            right.clone(),
        )
    }

    fn process_field_access(
        &mut self,
        target: &Expression<TypedNodeCommonFields>,
        field_name: &String,
    ) -> FieldAccess<TypedNodeCommonFields> {
        let target_type = match target.get_type() {
            Type::RecordType(r) => &**r,
            _ => unreachable!(),
        };
        let field_type = match target_type.fields.get(field_name) {
            Some(t) => t,
            None => panic!("Compilation error"),
        };

        FieldAccess::<TypedNodeCommonFields>::new(
            field_type.clone(),
            target.clone(),
            field_name.clone(),
        )
    }

    fn process_variable_declarator(
        &mut self,
        identifier: &Identifier,
        variable_type: &Type,
    ) -> VariableDeclarator<TypedNodeCommonFields> {
        // TODO(derekxu16): This should probably happen in a process_type_reference method instead
        // of here.
        let r#type = match variable_type {
            Type::TypeReference(r) => match self
                .type_environment
                .type_reference_to_record_type_converters
                .get(&(**r).identifier.name)
            {
                Some(converter) => *(**converter)(r),
                None => panic!("Compilation error"),
            },
            _ => (*variable_type).clone(),
        };
        VariableDeclarator::<TypedNodeCommonFields>::new(
            r#type,
            identifier.clone(),
            variable_type.clone(),
        )
    }

    fn after_process_variable_declarator(
        &mut self,
        variable_declarator: &VariableDeclarator<TypedNodeCommonFields>,
    ) -> () {
        self.environment_stack.top().insert(
            variable_declarator.identifier.name.clone(),
            variable_declarator.common_fields.r#type.clone(),
        );
    }

    fn process_parameter(
        &mut self,
        variable_declarator: &Rc<VariableDeclarator<TypedNodeCommonFields>>,
    ) -> Parameter<TypedNodeCommonFields> {
        Parameter::<TypedNodeCommonFields>::new(
            variable_declarator.common_fields.r#type.clone(),
            variable_declarator.clone(),
        )
    }

    fn process_block(
        &mut self,
        statements: &Vec<Statement<TypedNodeCommonFields>>,
        final_expression: &Option<Expression<TypedNodeCommonFields>>,
    ) -> Block<TypedNodeCommonFields> {
        Block::<TypedNodeCommonFields>::new(statements.clone(), final_expression.clone())
    }

    fn process_return_statement(
        &mut self,
        expression: &Expression<TypedNodeCommonFields>,
    ) -> ReturnStatement<TypedNodeCommonFields> {
        ReturnStatement::new(expression.clone())
    }

    fn before_process_function_declaration(
        &mut self,
        _function_declaration: &FunctionDeclaration<UntypedNodeCommonFields>,
    ) -> () {
        self.environment_stack.enter_scope();
    }

    fn process_function_declaration(
        &mut self,
        _function_declaration: &FunctionDeclaration<UntypedNodeCommonFields>,
        identifier: &Identifier,
        return_type: &Type,
        parameters: &Vec<Rc<Parameter<TypedNodeCommonFields>>>,
        body: &Rc<Block<TypedNodeCommonFields>>,
    ) -> FunctionDeclaration<TypedNodeCommonFields> {
        let parameter_types = parameters
            .iter()
            .map(|p| p.variable_declarator.common_fields.r#type.clone())
            .collect();

        let r#type = Type::FunctionType(Rc::new(FunctionType::new(
            parameter_types,
            match return_type {
                Type::TypeReference(r) => match self
                    .type_environment
                    .type_reference_to_record_type_converters
                    .get(&(**r).identifier.name)
                {
                    Some(converter) => *(**converter)(r),
                    None => panic!("Compilation error"),
                },
                _ => (*return_type).clone(),
            },
        )));

        FunctionDeclaration::<TypedNodeCommonFields>::new(
            r#type,
            identifier.clone(),
            return_type.clone(),
            parameters.clone(),
            body.clone(),
        )
    }

    fn after_process_function_declaration(
        &mut self,
        function_declaration: &FunctionDeclaration<TypedNodeCommonFields>,
    ) -> () {
        self.environment_stack.exit_scope();

        self.environment_stack.top().insert(
            function_declaration.identifier.name.clone(),
            function_declaration.common_fields.r#type.clone(),
        );
    }

    fn process_variable_declaration(
        &mut self,
        _variable_declaration: &VariableDeclaration<UntypedNodeCommonFields>,
        variable_declarator: &Rc<VariableDeclarator<TypedNodeCommonFields>>,
        initial_value: &Expression<TypedNodeCommonFields>,
    ) -> VariableDeclaration<TypedNodeCommonFields> {
        let initial_value_type = initial_value.get_type();
        if variable_declarator.common_fields.r#type != *initial_value_type {
            panic!("Compilation error: incompatible types")
        }

        VariableDeclaration::<TypedNodeCommonFields>::new(
            initial_value_type.clone(),
            variable_declarator.clone(),
            initial_value.clone(),
        )
    }

    fn process_source_file(
        &mut self,
        declarations: Vec<Declaration<TypedNodeCommonFields>>,
        type_declarations: Vec<ClassDeclaration>,
    ) -> SourceFile<TypedNodeCommonFields> {
        SourceFile::new(declarations, type_declarations)
    }
}
