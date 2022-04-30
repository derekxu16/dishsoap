use std::rc::Rc;

use crate::types::environment::{Environment, EnvironmentStack};
use crate::visitor::PostOrderVisitor;
use dishsoap_parser::ast::*;

pub struct TypeChecker {
    environment_stack: EnvironmentStack,
}

impl TypeChecker {
    pub fn new(initial_environment: Environment) -> TypeChecker {
        TypeChecker {
            environment_stack: EnvironmentStack::new(initial_environment),
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

    fn process_variable_reference(
        &mut self,
        variable_reference: &VariableReference<UntypedNodeCommonFields>,
    ) -> VariableReference<TypedNodeCommonFields> {
        let identifier = variable_reference.identifier.clone();
        VariableReference::<TypedNodeCommonFields>::new(
            self.environment_stack
                .top()
                .get(&identifier.name)
                .unwrap()
                .clone(),
            identifier,
        )
    }

    fn process_function_call(
        &mut self,
        _function_call: &FunctionCall<UntypedNodeCommonFields>,
        identifier: &Identifier,
        arguments: Vec<Expression<TypedNodeCommonFields>>,
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
            arguments,
        )
    }

    fn process_prefix_expression(
        &mut self,
        operator: &PrefixOperator,
        operand: &Expression<TypedNodeCommonFields>,
    ) -> PrefixExpression<TypedNodeCommonFields> {
        match operator {
            PrefixOperator::Bang => match operand.get_type() {
                Type::TypeLiteral(TypeLiteral::BoolType) => (),
                _ => panic!("Compilation error: incompatible types"),
            },
            PrefixOperator::Minus => match operand.get_type() {
                Type::TypeLiteral(TypeLiteral::I32Type) => (),
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

    fn process_if_expression(
        &mut self,
        condition: &Expression<TypedNodeCommonFields>,
        then_block: &Rc<Block<TypedNodeCommonFields>>,
        else_block: &Rc<Block<TypedNodeCommonFields>>,
    ) -> IfExpression<TypedNodeCommonFields> {
        // if then_block.c
        IfExpression::<TypedNodeCommonFields>::new(
            Type::TypeLiteral(TypeLiteral::I32Type),
            condition.clone(),
            then_block.clone(),
            else_block.clone(),
        )
    }

    fn process_variable_declarator(
        &mut self,
        _variable_declarator: &Rc<VariableDeclarator<UntypedNodeCommonFields>>,
        identifier: &Identifier,
        variable_type: &Type,
    ) -> VariableDeclarator<TypedNodeCommonFields> {
        VariableDeclarator::<TypedNodeCommonFields>::new(
            variable_type.clone(),
            identifier.clone(),
            variable_type.clone(),
        )
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
        function_declaration: &FunctionDeclaration<UntypedNodeCommonFields>,
    ) -> () {
        let environment = self.environment_stack.enter_scope();

        let parameter_types = function_declaration
            .parameters
            .iter()
            .map(|p| p.variable_declarator.variable_type.clone())
            .collect/* ::<Vec<Type>> */();
        environment.insert(
            function_declaration.identifier.name.clone(),
            Type::FunctionType(Rc::new(FunctionType::new(
                parameter_types,
                function_declaration.return_type.clone(),
            ))),
        );

        function_declaration.parameters.iter().for_each(|p| {
            let p_declarator = &p.variable_declarator;
            environment.insert(
                p_declarator.identifier.name.clone(),
                p_declarator.variable_type.clone(),
            );
        });
    }

    fn process_function_declaration(
        &mut self,
        _function_declaration: &FunctionDeclaration<UntypedNodeCommonFields>,
        identifier: &Identifier,
        return_type: &Type,
        parameters: &Vec<Rc<Parameter<TypedNodeCommonFields>>>,
        body: &Rc<Block<TypedNodeCommonFields>>,
    ) -> FunctionDeclaration<TypedNodeCommonFields> {
        FunctionDeclaration::<TypedNodeCommonFields>::new(
            Type::TypeLiteral(TypeLiteral::UnitType),
            identifier.clone(),
            return_type.clone(),
            parameters.clone(),
            body.clone(),
        )
    }

    fn after_process_function_declaration(&mut self) -> () {
        self.environment_stack.exit_scope();
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

    fn after_process_variable_declaration(
        &mut self,
        variable_declaration: &VariableDeclaration<UntypedNodeCommonFields>,
    ) -> () {
        let declarator = &variable_declaration.variable_declarator;
        self.environment_stack.top().insert(
            declarator.identifier.name.clone(),
            declarator.variable_type.clone(),
        );
    }

    fn process_source_file(
        &mut self,
        declarations: Vec<Rc<Declaration<TypedNodeCommonFields>>>,
    ) -> SourceFile<TypedNodeCommonFields> {
        SourceFile::new(declarations)
    }
}