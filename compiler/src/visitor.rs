use std::{collections::HashMap, ops::Deref, rc::Rc};

use dishsoap_parser::ast::*;

pub trait PostOrderVisitor<InputTypeCommonFields: Clone, ReturnTypeCommonFields: Clone> {
    fn process_unit_literal(&mut self) -> UnitLiteral<ReturnTypeCommonFields>;

    fn process_boolean_literal(&mut self, value: &bool) -> BooleanLiteral<ReturnTypeCommonFields>;

    fn process_integer_literal(&mut self, value: &i32) -> IntegerLiteral<ReturnTypeCommonFields>;

    fn process_object_literal(
        &mut self,
        class_name: &Identifier,
        fields: &HashMap<String, Expression<ReturnTypeCommonFields>>,
    ) -> ObjectLiteral<ReturnTypeCommonFields>;

    fn process_variable_reference(
        &mut self,
        identifier: &Identifier,
    ) -> VariableReference<ReturnTypeCommonFields>;

    fn process_function_call(
        &mut self,
        identifier: &Identifier,
        arguments: &Vec<Expression<ReturnTypeCommonFields>>,
    ) -> FunctionCall<ReturnTypeCommonFields>;

    fn process_if_expression(
        &mut self,
        condition: &Expression<ReturnTypeCommonFields>,
        then_block: &Rc<Block<ReturnTypeCommonFields>>,
        else_block: &Rc<Block<ReturnTypeCommonFields>>,
    ) -> IfExpression<ReturnTypeCommonFields>;

    fn process_prefix_expression(
        &mut self,
        operator: &PrefixOperator,
        operand: &Expression<ReturnTypeCommonFields>,
    ) -> PrefixExpression<ReturnTypeCommonFields>;

    fn process_binary_expression(
        &mut self,
        left: &Expression<ReturnTypeCommonFields>,
        operator: &InfixOperator,
        right: &Expression<ReturnTypeCommonFields>,
    ) -> BinaryExpression<ReturnTypeCommonFields>;

    fn process_field_access(
        &mut self,
        target: &Expression<ReturnTypeCommonFields>,
        field_name: &String,
    ) -> FieldAccess<ReturnTypeCommonFields>;

    fn process_expression(
        &mut self,
        expression: &Expression<InputTypeCommonFields>,
    ) -> Expression<ReturnTypeCommonFields> {
        match expression {
            Expression::UnitLiteral(_) => {
                Expression::UnitLiteral(Rc::new(self.process_unit_literal()))
            }
            Expression::BooleanLiteral(b) => {
                Expression::BooleanLiteral(Rc::new(self.process_boolean_literal(&b.value)))
            }
            Expression::IntegerLiteral(i) => {
                Expression::IntegerLiteral(Rc::new(self.process_integer_literal(&i.value)))
            }
            Expression::ObjectLiteral(r) => {
                let processed_fields = r
                    .fields
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            match self.visit(&Node::Expression(v.clone())) {
                                Node::Expression(e) => e,
                                _ => unreachable!(),
                            },
                        )
                    })
                    .collect();
                Expression::ObjectLiteral(Rc::new(
                    self.process_object_literal(&r.class_name, &processed_fields),
                ))
            }
            Expression::VariableReference(r) => Expression::VariableReference(Rc::new(
                self.process_variable_reference(&r.identifier),
            )),
            Expression::FunctionCall(c) => {
                let processed_arguments = c
                    .arguments
                    .iter()
                    .map(|a| match self.visit(&Node::Expression(a.deref().clone())) {
                        Node::Expression(e) => e,
                        _ => unreachable!(),
                    })
                    .collect();

                Expression::FunctionCall(Rc::new(
                    self.process_function_call(&c.identifier, &processed_arguments),
                ))
            }
            Expression::IfExpression(s) => {
                let processed_condition = match self.visit(&Node::Expression(s.condition.clone())) {
                    Node::Expression(e) => e,
                    _ => unreachable!(),
                };
                let processed_then_block = match self.visit(&Node::Block(s.then_block.clone())) {
                    Node::Block(b) => b,
                    _ => unreachable!(),
                };
                let processed_else_block = match self.visit(&Node::Block(s.else_block.clone())) {
                    Node::Block(b) => b,
                    _ => unreachable!(),
                };

                Expression::IfExpression(Rc::new(self.process_if_expression(
                    &processed_condition,
                    &processed_then_block,
                    &processed_else_block,
                )))
            }
            Expression::PrefixExpression(e) => {
                let processed_operand = match self.visit(&Node::Expression(e.operand.clone())) {
                    Node::Expression(e) => e,
                    _ => unreachable!(),
                };

                Expression::PrefixExpression(Rc::new(
                    self.process_prefix_expression(&e.operator, &processed_operand),
                ))
            }
            Expression::BinaryExpression(e) => {
                let processed_left = match self.visit(&Node::Expression(e.left.clone())) {
                    Node::Expression(e) => e,
                    _ => unreachable!(),
                };
                let processed_right = match self.visit(&Node::Expression(e.right.clone())) {
                    Node::Expression(e) => e,
                    _ => unreachable!(),
                };

                Expression::BinaryExpression(Rc::new(self.process_binary_expression(
                    &Rc::new(processed_left),
                    &e.operator,
                    &Rc::new(processed_right),
                )))
            }
            Expression::FieldAccess(a) => {
                let processed_target = match self.visit(&Node::Expression(a.target.clone())) {
                    Node::Expression(e) => e,
                    _ => unreachable!(),
                };

                Expression::FieldAccess(Rc::new(
                    self.process_field_access(&processed_target, &a.field_name),
                ))
            }
        }
    }

    fn process_variable_declarator(
        &mut self,
        identifier: &Identifier,
        variable_type: &Type,
    ) -> VariableDeclarator<ReturnTypeCommonFields>;

    fn process_parameter(
        &mut self,
        variable_declarator: &Rc<VariableDeclarator<ReturnTypeCommonFields>>,
    ) -> Parameter<ReturnTypeCommonFields>;

    fn process_block(
        &mut self,
        statements: &Vec<Statement<ReturnTypeCommonFields>>,
        final_expression: &Option<Expression<ReturnTypeCommonFields>>,
    ) -> Block<ReturnTypeCommonFields>;

    fn process_return_statement(
        &mut self,
        expression: &Expression<ReturnTypeCommonFields>,
    ) -> ReturnStatement<ReturnTypeCommonFields>;

    fn before_process_function_declaration(
        &mut self,
        _function_declaration: &FunctionDeclaration<InputTypeCommonFields>,
    ) -> () {
    }

    fn process_function_declaration(
        &mut self,
        function_declaration: &FunctionDeclaration<InputTypeCommonFields>,
        identifier: &Identifier,
        return_type: &Type,
        parameters: &Vec<Rc<Parameter<ReturnTypeCommonFields>>>,
        body: &Rc<Block<ReturnTypeCommonFields>>,
    ) -> FunctionDeclaration<ReturnTypeCommonFields>;

    fn after_process_function_declaration(&mut self) -> () {}

    fn before_process_variable_declaration(
        &mut self,
        _variable_declaration: &VariableDeclaration<InputTypeCommonFields>,
    ) -> () {
    }

    fn process_variable_declaration(
        &mut self,
        variable_declaration: &VariableDeclaration<InputTypeCommonFields>,
        variable_declarator: &Rc<VariableDeclarator<ReturnTypeCommonFields>>,
        initial_value: &Expression<ReturnTypeCommonFields>,
    ) -> VariableDeclaration<ReturnTypeCommonFields>;

    fn after_process_variable_declaration(
        &mut self,
        _variable_declaration: &VariableDeclaration<ReturnTypeCommonFields>,
    ) -> () {
    }

    fn process_statement(
        &mut self,
        statement: &Statement<InputTypeCommonFields>,
    ) -> Statement<ReturnTypeCommonFields> {
        match statement {
            Statement::Declaration(d) => match d {
                Declaration::VariableDeclaration(vd) => {
                    self.before_process_variable_declaration(vd);

                    let processed_variable_declarator = match self
                        .visit(&Node::VariableDeclarator(vd.variable_declarator.clone()))
                    {
                        Node::VariableDeclarator(vd) => vd,
                        _ => unreachable!(),
                    };
                    let processed_initial_value =
                        match self.visit(&Node::Expression(vd.initial_value.clone())) {
                            Node::Expression(e) => e,
                            _ => unreachable!(),
                        };

                    let processed_variable_declaration = self.process_variable_declaration(
                        &vd,
                        &processed_variable_declarator,
                        &processed_initial_value,
                    );

                    self.after_process_variable_declaration(&processed_variable_declaration);

                    Statement::Declaration(Declaration::VariableDeclaration(Rc::new(
                        processed_variable_declaration,
                    )))
                }
                Declaration::FunctionDeclaration(fd) => {
                    self.before_process_function_declaration(fd);

                    let processed_parameters = fd
                        .parameters
                        .iter()
                        .map(|p| match self.visit(&Node::Parameter(p.clone())) {
                            Node::Parameter(tp) => tp,
                            _ => unreachable!(),
                        })
                        .collect();
                    let processed_body = match self.visit(&Node::Block(fd.body.clone())) {
                        Node::Block(b) => b.to_owned(),
                        _ => unreachable!(),
                    };

                    let processed_function_declaration = self.process_function_declaration(
                        fd,
                        &fd.identifier,
                        &fd.return_type,
                        &processed_parameters,
                        &processed_body,
                    );

                    self.after_process_function_declaration();

                    Statement::Declaration(Declaration::FunctionDeclaration(Rc::new(
                        processed_function_declaration,
                    )))
                }
                _ => unreachable!(),
            },
            Statement::ReturnStatement(s) => {
                let processed_expression = match self.visit(&Node::Expression(s.expression.clone()))
                {
                    Node::Expression(e) => e,
                    _ => unreachable!(),
                };
                Statement::ReturnStatement(Rc::new(
                    self.process_return_statement(&processed_expression),
                ))
            }
        }
    }

    fn process_source_file(
        &mut self,
        declarations: Vec<Declaration<ReturnTypeCommonFields>>,
        type_declarations: Vec<ClassDeclaration>,
    ) -> SourceFile<ReturnTypeCommonFields>;

    /// This method handles traversal of an AST subtree. This should never be re-implemented,
    /// functionality should only be implemented in `process` methods.
    fn visit(&mut self, n: &Node<InputTypeCommonFields>) -> Node<ReturnTypeCommonFields> {
        match n {
            Node::Identifier(_) => unreachable!(),
            Node::Type(_) => unreachable!(),
            Node::Expression(e) => Node::Expression(self.process_expression(&e)),
            Node::VariableDeclarator(d) => Node::VariableDeclarator(Rc::new(
                self.process_variable_declarator(&d.identifier, &d.variable_type),
            )),
            Node::Parameter(p) => {
                let processed_variable_declarator =
                    match self.visit(&Node::VariableDeclarator(p.variable_declarator.clone())) {
                        Node::VariableDeclarator(vd) => vd,
                        _ => unreachable!(),
                    };

                Node::Parameter(Rc::new(
                    self.process_parameter(&processed_variable_declarator),
                ))
            }
            Node::Block(b) => {
                let processed_statements = b
                    .statements
                    .iter()
                    .map(|s| match self.visit(&Node::Statement(s.clone())) {
                        Node::Statement(s) => s,
                        _ => unreachable!(),
                    })
                    .collect();
                let processed_final_expresion = match &b.final_expression {
                    Some(fe) => match self.visit(&Node::Expression(fe.deref().clone())) {
                        Node::Expression(e) => Some(e),
                        _ => unreachable!(),
                    },
                    None => None,
                };

                Node::Block(Rc::new(
                    self.process_block(&processed_statements, &processed_final_expresion),
                ))
            }
            Node::Statement(s) => Node::Statement(self.process_statement(s)),
            Node::SourceFile(sf) => {
                let processed_declarations = sf
                    .declarations
                    .iter()
                    .map(|d| {
                        match self.visit(&Node::Statement(Statement::Declaration((*d).clone()))) {
                            Node::Statement(Statement::Declaration(d)) => d,
                            _ => unreachable!(),
                        }
                    })
                    .collect();

                Node::<ReturnTypeCommonFields>::SourceFile(Rc::new(
                    self.process_source_file(processed_declarations, sf.type_declarations.clone()),
                ))
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct PreOrderVisitorResponse {
    should_stop_traversing: bool,
}

impl PreOrderVisitorResponse {
    const FLYWEIGHT_STOP_TRAVERSING_RESPONSE: PreOrderVisitorResponse = PreOrderVisitorResponse {
        should_stop_traversing: true,
    };
    const FLYWEIGHT_DO_NOT_STOP_TRAVERSING_RESPONSE: PreOrderVisitorResponse =
        PreOrderVisitorResponse {
            should_stop_traversing: false,
        };

    pub fn new(should_stop_traversing: bool) -> &'static PreOrderVisitorResponse {
        if should_stop_traversing {
            &PreOrderVisitorResponse::FLYWEIGHT_STOP_TRAVERSING_RESPONSE
        } else {
            &PreOrderVisitorResponse::FLYWEIGHT_DO_NOT_STOP_TRAVERSING_RESPONSE
        }
    }
}

pub trait PreOrderVisitor<InputTypeCommonFields: Clone> {
    fn process_boolean_literal(
        &mut self,
        _boolean_literal: &BooleanLiteral<InputTypeCommonFields>,
    ) -> () {
    }

    fn process_integer_literal(
        &mut self,
        _integer_literal: &IntegerLiteral<InputTypeCommonFields>,
    ) -> () {
    }

    fn process_object_literal(
        &mut self,
        _struct_literal: &ObjectLiteral<InputTypeCommonFields>,
    ) -> () {
    }

    fn process_variable_reference(
        &mut self,
        _variable_reference: &VariableReference<InputTypeCommonFields>,
    ) -> () {
    }

    fn process_function_call(
        &mut self,
        _function_call: &FunctionCall<InputTypeCommonFields>,
    ) -> PreOrderVisitorResponse {
        *PreOrderVisitorResponse::new(false)
    }

    fn process_if_expression(
        &mut self,
        _if_expression: &IfExpression<InputTypeCommonFields>,
    ) -> PreOrderVisitorResponse {
        *PreOrderVisitorResponse::new(false)
    }

    fn process_prefix_expression(
        &mut self,
        _prefix_expression: &PrefixExpression<InputTypeCommonFields>,
    ) -> PreOrderVisitorResponse {
        *PreOrderVisitorResponse::new(false)
    }

    fn process_binary_expression(
        &mut self,
        _binary_expression: &BinaryExpression<InputTypeCommonFields>,
    ) -> PreOrderVisitorResponse {
        *PreOrderVisitorResponse::new(false)
    }

    fn process_field_access(&mut self, _field_access: &FieldAccess<InputTypeCommonFields>) -> () {}

    fn process_expression(&mut self, expression: &Expression<InputTypeCommonFields>) -> () {
        match expression {
            Expression::UnitLiteral(_) => (),
            Expression::BooleanLiteral(b) => self.process_boolean_literal(&**b),
            Expression::IntegerLiteral(i) => self.process_integer_literal(&**i),
            Expression::ObjectLiteral(r) => self.process_object_literal(&**r),
            Expression::VariableReference(r) => self.process_variable_reference(&**r),
            Expression::FunctionCall(c) => {
                if !self.process_function_call(&**c).should_stop_traversing {
                    c.arguments
                        .iter()
                        .for_each(|a| self.visit(&Node::Expression(a.deref().clone())));
                }
            }
            Expression::IfExpression(e) => {
                if !self.process_if_expression(&**e).should_stop_traversing {
                    self.visit(&Node::Expression(e.condition.clone()));
                    self.visit(&Node::Block(e.then_block.clone()));
                    self.visit(&Node::Block(e.else_block.clone()));
                }
            }
            Expression::PrefixExpression(e) => {
                if !self.process_prefix_expression(&**e).should_stop_traversing {
                    self.visit(&Node::Expression(e.operand.clone()));
                }
            }
            Expression::BinaryExpression(e) => {
                if !self.process_binary_expression(&**e).should_stop_traversing {
                    self.visit(&Node::Expression(e.left.clone()));
                    self.visit(&Node::Expression(e.right.clone()));
                }
            }
            Expression::FieldAccess(a) => self.process_field_access(&**a),
        };
    }

    fn process_variable_declarator(
        &mut self,
        _variable_declarator: &VariableDeclarator<InputTypeCommonFields>,
    ) -> () {
    }

    fn process_parameter(
        &mut self,
        _parameter: &Parameter<InputTypeCommonFields>,
    ) -> PreOrderVisitorResponse {
        *PreOrderVisitorResponse::new(false)
    }

    fn process_block(&mut self, _block: &Block<InputTypeCommonFields>) -> PreOrderVisitorResponse {
        *PreOrderVisitorResponse::new(false)
    }

    fn process_return_statement(
        &mut self,
        _return_statement: &ReturnStatement<InputTypeCommonFields>,
    ) -> PreOrderVisitorResponse {
        *PreOrderVisitorResponse::new(false)
    }

    fn process_function_declaration(
        &mut self,
        _function_declaration: &FunctionDeclaration<InputTypeCommonFields>,
    ) -> PreOrderVisitorResponse {
        *PreOrderVisitorResponse::new(false)
    }

    fn process_variable_declaration(
        &mut self,
        _variable_declaration: &VariableDeclaration<InputTypeCommonFields>,
    ) -> PreOrderVisitorResponse {
        *PreOrderVisitorResponse::new(false)
    }

    fn process_statement(&mut self, statement: &Statement<InputTypeCommonFields>) -> () {
        match statement {
            Statement::Declaration(d) => match d {
                Declaration::VariableDeclaration(vd) => {
                    if !self.process_variable_declaration(vd).should_stop_traversing {
                        self.visit(&Node::VariableDeclarator(vd.variable_declarator.clone()));
                        self.visit(&Node::Expression(vd.initial_value.clone()));
                    }
                }
                Declaration::FunctionDeclaration(fd) => {
                    if !self
                        .process_function_declaration(&fd)
                        .should_stop_traversing
                    {
                        fd.parameters.iter().for_each(|p| {
                            self.visit(&Node::Parameter(p.clone()));
                        });
                        self.visit(&Node::Block(fd.body.clone()));
                    }
                }
                _ => unreachable!(),
            },
            Statement::ReturnStatement(s) => {
                if !self.process_return_statement(&**s).should_stop_traversing {
                    self.visit(&Node::Expression(s.expression.clone()));
                }
            }
        };
    }

    fn process_source_file(
        &mut self,
        _source_file: &SourceFile<InputTypeCommonFields>,
    ) -> PreOrderVisitorResponse {
        *PreOrderVisitorResponse::new(false)
    }

    /// This method handles traversal of an AST subtree. This should never be re-implemented,
    /// functionality should only be implemented in `process` methods.
    fn visit<'a>(&mut self, n: &Node<InputTypeCommonFields>) -> () {
        match n {
            Node::Identifier(_) => unreachable!(),
            Node::Type(_) => unreachable!(),
            Node::Expression(e) => {
                self.process_expression(e);
            }
            Node::VariableDeclarator(d) => {
                self.process_variable_declarator(&**d);
            }
            Node::Parameter(p) => {
                if !self.process_parameter(&**p).should_stop_traversing {
                    self.visit(&Node::VariableDeclarator(p.variable_declarator.clone()));
                }
            }
            Node::Block(b) => {
                if !self.process_block(&**b).should_stop_traversing {
                    b.statements
                        .iter()
                        .for_each(|s| self.visit(&Node::Statement(s.deref().clone())));
                    match &b.final_expression {
                        Some(fe) => self.visit(&Node::Expression(fe.deref().clone())),
                        None => (),
                    };
                }
            }
            Node::Statement(s) => {
                self.process_statement(s);
            }
            Node::SourceFile(sf) => {
                if !self.process_source_file(&**sf).should_stop_traversing {
                    sf.declarations.iter().for_each(|d| {
                        self.visit(&Node::Statement(Statement::Declaration(
                            d.deref().deref().clone(),
                        )));
                    });
                }
            }
        };
    }
}
