pub mod ast;
pub mod test_inputs;
use std::{collections::HashMap, rc::Rc};

use ast::*;
use dishsoap_lexer::{Lexer, Token};

pub struct Parser<'ast> {
    lexer: Lexer<'ast>,
    scope_depth: i32,
}

impl<'ast> Parser<'ast> {
    pub fn new(source: &'ast str) -> Self {
        let parser = Parser {
            lexer: Lexer::new(source),
            scope_depth: 0,
        };

        parser
    }

    /// Used to keep track of the number of opening braces encountered.
    fn increase_scope_depth(&mut self) {
        self.scope_depth += 1;
    }

    /// Used to keep track of the number of closing braces encountered.
    fn decrease_scope_depth(&mut self) {
        self.scope_depth -= 1;
    }

    /// Get the current scope depth.
    fn get_scope_depth(&self) -> i32 {
        self.scope_depth
    }

    fn parse_identifier(&mut self) -> Identifier {
        match self.lexer.consume(Token::Identifier) {
            Err(e) => panic!("{}", e.message),
            Ok(Token::Identifier) => Identifier::new(self.lexer.slice().to_owned()),
            _ => unreachable!(),
        }
    }

    fn parse_type_reference(&mut self) -> TypeReference {
        let identifier = self.parse_identifier();

        TypeReference::new(identifier)
    }

    fn parse_type(&mut self) -> Type {
        match self.lexer.peek() {
            Some(Token::UnitPrimitiveKeyword)
            | Some(Token::BoolPrimitiveKeyword)
            | Some(Token::I32PrimitiveKeyword) => match self.lexer.pop() {
                Some(Token::UnitPrimitiveKeyword) => Type::UnitType,
                Some(Token::BoolPrimitiveKeyword) => Type::BoolType,
                Some(Token::I32PrimitiveKeyword) => Type::I32Type,
                _ => panic!("Compilation error"),
            },
            Some(Token::Identifier) => Type::TypeReference(Rc::new(self.parse_type_reference())),
            Some(Token::ParenOpen) => todo!("Support function type annotations"),
            _ => panic!("Compilation error"),
        }
    }

    fn parse_boolean_literal(&mut self) -> BooleanLiteral<UntypedNodeCommonFields> {
        match self.lexer.pop() {
            Some(Token::TrueKeyword) => BooleanLiteral::<UntypedNodeCommonFields>::new(true),
            Some(Token::FalseKeyword) => BooleanLiteral::<UntypedNodeCommonFields>::new(false),
            _ => panic!("Compilation error: unexpected token"),
        }
    }

    fn parse_integer_literal(&mut self) -> IntegerLiteral<UntypedNodeCommonFields> {
        match self.lexer.consume(Token::IntegerLiteral) {
            Err(e) => panic!("{}", e.message),
            _ => {
                let value: String = self.lexer.slice().to_owned();
                IntegerLiteral::<UntypedNodeCommonFields>::new(value.parse::<i32>().unwrap())
            }
        }
    }

    fn parse_object_literal(
        &mut self,
        class_name: Identifier,
    ) -> ObjectLiteral<UntypedNodeCommonFields> {
        match self.lexer.consume(Token::BraceOpen) {
            Err(e) => panic!("{}", e.message),
            _ => (),
        }

        let mut fields = HashMap::new();
        loop {
            if self.lexer.peek() == Some(Token::BraceClose) {
                let _ = self.lexer.consume(Token::BraceClose);
                break;
            }

            let field_name = self.parse_identifier();
            match self.lexer.consume(Token::Colon) {
                Err(e) => panic!("{}", e.message),
                _ => (),
            }
            let field_value = match self.parse_expression(0) {
                Some(e) => e,
                _ => panic!("Compilation error"),
            };

            fields.insert(field_name.name, field_value);

            if self.lexer.peek() == Some(Token::Comma) {
                let _ = self.lexer.consume(Token::Comma);
            }
        }

        ObjectLiteral::<UntypedNodeCommonFields>::new(class_name, fields)
    }

    /**
     * Tries to parse an ObjectLiteral, FunctionCall, or VariableReference. Returns None if
     * unsuccessful.
     */
    fn parse_object_literal_or_function_call_or_variable_reference(
        &mut self,
    ) -> Expression<UntypedNodeCommonFields> {
        let identifier = self.parse_identifier();
        match self.lexer.peek() {
            Some(Token::BraceOpen) => {
                Expression::ObjectLiteral(Rc::new(self.parse_object_literal(identifier)))
            }
            Some(Token::ParenOpen) => {
                let arguments = self.parse_arguments();
                Expression::FunctionCall(Rc::new(FunctionCall::<UntypedNodeCommonFields>::new(
                    identifier, arguments,
                )))
            }
            _ => Expression::<UntypedNodeCommonFields>::VariableReference(Rc::new(
                VariableReference::<UntypedNodeCommonFields>::new(identifier),
            )),
        }
    }

    fn parse_if_expression(&mut self) -> IfExpression<UntypedNodeCommonFields> {
        match self.lexer.consume(Token::IfKeyword) {
            Err(e) => panic!("{}", e.message),
            _ => (),
        }
        match self.lexer.consume(Token::ParenOpen) {
            Err(e) => panic!("{}", e.message),
            _ => (),
        }
        let condition = self.parse_expression(0);
        if condition.is_none() {
            panic!("Compilation error");
        }
        match self.lexer.consume(Token::ParenClose) {
            Err(e) => panic!("{}", e.message),
            _ => (),
        }

        let then_block = self.parse_block();
        let else_block = match self.lexer.peek() {
            Some(Token::ElseKeyword) => {
                let _ = self.lexer.consume(Token::ElseKeyword);
                self.parse_block()
            }
            _ => Block::new_no_final_expression(vec![]),
        };

        IfExpression::<UntypedNodeCommonFields>::new(
            condition.unwrap(),
            Rc::new(then_block),
            Rc::new(else_block),
        )
    }

    fn parse_prefix_operator(&mut self) -> PrefixOperator {
        match self.lexer.pop() {
            // Some(Token::Plus) => PrefixOperator::Plus,
            Some(Token::Minus) => PrefixOperator::Minus,
            Some(Token::Bang) => PrefixOperator::Bang,
            _ => panic!("Compilation error: unexpected token"),
        }
    }

    fn parse_infix_operator(&mut self) -> InfixOperator {
        match self.lexer.pop() {
            Some(Token::DoubleEquals) => InfixOperator::DoubleEquals,
            Some(Token::LessThan) => InfixOperator::LessThan,
            Some(Token::LessThanEquals) => InfixOperator::LessThanEquals,
            Some(Token::GreaterThan) => InfixOperator::GreaterThan,
            Some(Token::GreaterThanEquals) => InfixOperator::GreaterThanEquals,
            Some(Token::Plus) => InfixOperator::Plus,
            Some(Token::Minus) => InfixOperator::Minus,
            Some(Token::Times) => InfixOperator::Times,
            Some(Token::Divide) => InfixOperator::Divide,
            Some(Token::Percent) => InfixOperator::Modulo,
            Some(Token::Dot) => InfixOperator::Dot,
            _ => panic!("Compilation error: unexpected token"),
        }
    }

    /// Helper function to get operator precedence for Pratt parsing.
    fn get_precedence(&mut self) -> i32 {
        match self.lexer.peek() {
            Some(Token::Equals) => 1,
            Some(Token::DoubleEquals) => 2,
            Some(Token::LessThan)
            | Some(Token::LessThanEquals)
            | Some(Token::GreaterThan)
            | Some(Token::GreaterThanEquals) => 3,
            Some(Token::Plus) | Some(Token::Minus) => 4,
            Some(Token::Times) | Some(Token::Divide) | Some(Token::Percent) => 5,
            Some(Token::Dot) => 6,
            _ => 0,
        }
    }

    fn parse_prefix_expression(&mut self) -> PrefixExpression<UntypedNodeCommonFields> {
        let operator = self.parse_prefix_operator();
        let operand = self.parse_expression(0);
        if operand.is_none() {
            panic!("Compilation error");
        }

        PrefixExpression::<UntypedNodeCommonFields>::new(operator, operand.unwrap())
    }

    /// Parses an expression.
    // Uses Pratt parsing to handle operator precedence.
    // http://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/
    fn parse_expression(&mut self, precedence: i32) -> Option<Expression<UntypedNodeCommonFields>> {
        let mut left = match self.lexer.peek() {
            Some(Token::TrueKeyword) | Some(Token::FalseKeyword) => Some(
                Expression::BooleanLiteral(Rc::new(self.parse_boolean_literal())),
            ),
            Some(Token::IntegerLiteral) => Some(Expression::IntegerLiteral(Rc::new(
                self.parse_integer_literal(),
            ))),
            Some(Token::Identifier) => {
                Some(self.parse_object_literal_or_function_call_or_variable_reference())
            }
            Some(Token::IfKeyword) => Some(Expression::IfExpression(Rc::new(
                self.parse_if_expression(),
            ))),
            Some(Token::Plus) | Some(Token::Minus) | Some(Token::Bang) => Some(
                Expression::PrefixExpression(Rc::new(self.parse_prefix_expression())),
            ),
            Some(Token::ParenOpen) => {
                let _ = self.lexer.consume(Token::ParenOpen);
                let expression: Option<Expression<UntypedNodeCommonFields>> =
                    self.parse_expression(precedence);
                if expression.is_none() {
                    panic!("Compilation error");
                }
                match self.lexer.consume(Token::ParenClose) {
                    Err(e) => panic!("{}", e.message),
                    _ => (),
                }

                expression
            }
            _ => None,
        };

        loop {
            let next_precedence = self.get_precedence();
            if next_precedence <= precedence {
                // If the precedence of the next operator is too low, defer parsing.
                break;
            }

            let infix_operator = self.parse_infix_operator();

            left = match infix_operator {
                InfixOperator::Dot => Some(Expression::FieldAccess(Rc::new(FieldAccess::<
                    UntypedNodeCommonFields,
                >::new(
                    left.unwrap(),
                    self.parse_identifier().name,
                )))),
                _ => Some(Expression::BinaryExpression(Rc::new(BinaryExpression::<
                    UntypedNodeCommonFields,
                >::new(
                    left.unwrap(),
                    infix_operator,
                    self.parse_expression(next_precedence).unwrap(),
                )))),
            }
        }
        left
    }

    fn parse_arguments(&mut self) -> Vec<Expression<UntypedNodeCommonFields>> {
        let mut arguments: Vec<Expression<UntypedNodeCommonFields>> = Vec::new();

        match self.lexer.consume(Token::ParenOpen) {
            Err(e) => panic!("{}", e.message),
            _ => (),
        }

        loop {
            if self.lexer.peek() == Some(Token::ParenClose) {
                let _ = self.lexer.consume(Token::ParenClose);
                break;
            }

            let argument = self.parse_expression(0);
            if argument.is_none() {
                panic!("Compilation error");
            }
            arguments.push(argument.unwrap());

            if self.lexer.peek() == Some(Token::Comma) {
                let _ = self.lexer.consume(Token::Comma);
            }
        }
        arguments
    }

    fn parse_variable_declarator(&mut self) -> VariableDeclarator<UntypedNodeCommonFields> {
        match self.lexer.peek() {
            Some(Token::Identifier) => {
                let identifier = self.parse_identifier();

                match self.lexer.consume(Token::Colon) {
                    Err(e) => panic!("{}", e.message),
                    _ => (),
                }

                let variable_type = self.parse_type();

                VariableDeclarator::<UntypedNodeCommonFields> {
                    common_fields: UntypedNodeCommonFields::new(),
                    identifier,
                    variable_type,
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse_parameter(&mut self) -> Parameter<UntypedNodeCommonFields> {
        match self.lexer.peek() {
            Some(Token::Identifier) => {
                let variable_declarator = self.parse_variable_declarator();

                Parameter::<UntypedNodeCommonFields>::new(Rc::new(variable_declarator))
            }
            _ => unreachable!(),
        }
    }

    fn parse_parameters(&mut self) -> Vec<Parameter<UntypedNodeCommonFields>> {
        let mut parameters: Vec<Parameter<UntypedNodeCommonFields>> = Vec::new();

        match self.lexer.consume(Token::ParenOpen) {
            Err(e) => panic!("{}", e.message),
            _ => (),
        }

        loop {
            if self.lexer.peek() == Some(Token::ParenClose) {
                let _ = self.lexer.consume(Token::ParenClose);
                break;
            }

            let parameter = self.parse_parameter();
            parameters.push(parameter);

            if self.lexer.peek() == Some(Token::Comma) {
                let _ = self.lexer.consume(Token::Comma);
            }
        }
        parameters
    }

    fn parse_block(&mut self) -> Block<UntypedNodeCommonFields> {
        let target_depth: i32 = self.get_scope_depth();

        if self.lexer.peek() == Some(Token::BraceOpen) {
            let _ = self.lexer.consume(Token::BraceOpen);
            self.increase_scope_depth();
        }

        let mut statements: Vec<Statement<UntypedNodeCommonFields>> = Vec::new();
        let mut maybe_final_expression;

        loop {
            let statement = self.parse_statement();
            if statement.is_some() {
                statements.push(statement.unwrap());
                continue;
            } else {
                maybe_final_expression = self.parse_expression(0);
                match self.lexer.peek() {
                    Some(Token::BraceClose) => {
                        let _ = self.lexer.consume(Token::BraceClose);
                        self.decrease_scope_depth();
                        if self.get_scope_depth() == target_depth {
                            break;
                        }
                    }
                    _ => (),
                }
            }
        }

        match maybe_final_expression {
            Some(e) => Block::new_with_final_expression(statements, e),
            None => Block::new_no_final_expression(statements),
        }
    }

    // TODO(derekxu16): Support return statements.
    // fn parse_return_statement(&mut self) -> ReturnStatement<UntypedNodeCommonFields> {
    //     match self.lexer.consume(Token::ReturnKeyword) {
    //         Err(e) => panic!("{}", e.message),
    //         _ => (),
    //     }

    //     let expression = match self.parse_expression(0) {
    //         Some(e) => ReturnStatement::new(e),
    //         _ => {
    //             panic!("Compilation error");
    //         }
    //     };

    //     match self.lexer.consume(Token::Semicolon) {
    //         Err(e) => panic!("{}", e.message),
    //         _ => (),
    //     }

    //     expression
    // }

    fn parse_class_declaration(&mut self) -> ClassDeclaration {
        match self.lexer.consume(Token::ClassKeyword) {
            Err(e) => panic!("{}", e.message),
            _ => (),
        }
        let identifier = self.parse_identifier();

        let mut fields = HashMap::new();
        match self.lexer.consume(Token::BraceOpen) {
            Err(e) => panic!("{}", e.message),
            _ => (),
        }
        loop {
            if self.lexer.peek() == Some(Token::BraceClose) {
                let _ = self.lexer.consume(Token::BraceClose);
                break;
            }

            let field_name = self.parse_identifier();
            match self.lexer.consume(Token::Colon) {
                Err(e) => panic!("{}", e.message),
                _ => (),
            }
            let field_type = self.parse_type();
            fields.insert(field_name.name, field_type);

            if self.lexer.peek() == Some(Token::Comma) {
                let _ = self.lexer.consume(Token::Comma);
            }
        }

        ClassDeclaration::new(identifier, fields)
    }

    fn parse_function_declaration(&mut self) -> FunctionDeclaration<UntypedNodeCommonFields> {
        match self.lexer.consume(Token::FuncKeyword) {
            Err(e) => panic!("{}", e.message),
            _ => (),
        }

        let identifier = self.parse_identifier();
        let parameters = self
            .parse_parameters()
            .iter()
            .map(|p| Rc::new(p.clone()))
            .collect();

        match self.lexer.consume(Token::Arrow) {
            Err(e) => panic!("{}", e.message),
            _ => (),
        }

        let return_type = self.parse_type();

        FunctionDeclaration::<UntypedNodeCommonFields>::new(
            identifier,
            return_type,
            parameters,
            Rc::new(self.parse_block()),
        )
    }

    fn parse_variable_declaration(&mut self) -> VariableDeclaration<UntypedNodeCommonFields> {
        match self.lexer.consume(Token::LetKeyword) {
            Err(e) => panic!("{}", e.message),
            _ => (),
        }

        let variable_declarator = self.parse_variable_declarator();

        match self.lexer.consume(Token::Equals) {
            Err(e) => panic!("{}", e.message),
            _ => (),
        }

        let initial_value = self.parse_expression(0);
        if initial_value.is_none() {
            panic!("Compilation error: unexpected token");
        }

        match self.lexer.consume(Token::Semicolon) {
            Err(e) => panic!("{}", e.message),
            _ => (),
        }

        VariableDeclaration::<UntypedNodeCommonFields>::new(
            Rc::new(variable_declarator),
            initial_value.unwrap(),
        )
    }

    fn parse_statement(&mut self) -> Option<Statement<UntypedNodeCommonFields>> {
        match self.lexer.peek() {
            Some(Token::LetKeyword) => Some(Statement::Declaration(
                Declaration::VariableDeclaration(Rc::new(self.parse_variable_declaration())),
            )),
            Some(Token::ReturnKeyword) => {
                todo!("Support return statements")
                // Some(Statement::ReturnStatement(self.parse_return_statement()))
            }
            _ => None,
        }
    }

    fn parse_source_file(&mut self) -> SourceFile<UntypedNodeCommonFields> {
        let mut declarations: Vec<Declaration<UntypedNodeCommonFields>> = Vec::new();
        let mut type_declarations: Vec<ClassDeclaration> = Vec::new();

        loop {
            let t = self.lexer.peek();
            match t {
                Some(Token::ClassKeyword) => type_declarations.push(self.parse_class_declaration()),
                Some(Token::FuncKeyword) => declarations.push(Declaration::FunctionDeclaration(
                    Rc::new(self.parse_function_declaration()),
                )),
                None => {
                    // EOF
                    break;
                }
                _ => panic!("Compilation error: unexpected token"),
            }
        }

        SourceFile::new(declarations, type_declarations)
    }

    /// Parses a file and returns a [Node::SourceFile]. This is the entry point for parsing an
    /// entire file.
    pub fn parse(&mut self) -> Node<UntypedNodeCommonFields> {
        Node::SourceFile(Rc::new(self.parse_source_file()))
    }
}

#[cfg(test)]
mod tests;
