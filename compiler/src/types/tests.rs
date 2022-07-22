use crate::types::gather_top_level_declarations::gather_top_level_declarations;
use crate::types::type_checker::TypeChecker;
use crate::visitor::PostOrderVisitor;
use dishsoap_parser::ast::*;
use dishsoap_parser::test_inputs;
use dishsoap_parser::Parser;
use std::collections::HashMap;
use std::rc::Rc;

mod tests {
    use super::*;

    fn define_test_body(body: Rc<Block<TypedNodeCommonFields>>) -> Node<TypedNodeCommonFields> {
        Node::SourceFile(Rc::new(SourceFile::new(vec![Rc::new(
            Declaration::FunctionDeclaration(Rc::new(
                FunctionDeclaration::<TypedNodeCommonFields>::new(
                    Type::UnitType,
                    Identifier::new("test".to_owned()),
                    Type::I32Type,
                    vec![],
                    body,
                ),
            )),
        )])))
    }

    fn parse_and_check(source: &str) -> Node<TypedNodeCommonFields> {
        let mut parser = Parser::new(source);
        let untyped_ast = parser.parse();
        let mut type_checker = TypeChecker::new(gather_top_level_declarations(&untyped_ast));
        let typed_ast = type_checker.visit(&untyped_ast).clone();

        typed_ast
    }

    #[test]
    fn prefix_expression_not() {
        let sf_node = parse_and_check(test_inputs::PREFIX_OPERATION_NOT);
        assert_eq!(
            sf_node,
            Node::SourceFile(Rc::new(SourceFile::new(vec![Rc::new(
                Declaration::FunctionDeclaration(Rc::new(FunctionDeclaration::<
                    TypedNodeCommonFields,
                >::new(
                    Type::UnitType,
                    Identifier::new("test".to_owned()),
                    Type::BoolType,
                    vec![],
                    Rc::new(Block::new_with_final_expression(
                        vec![],
                        Expression::PrefixExpression(Rc::new(PrefixExpression::<
                            TypedNodeCommonFields,
                        >::new(
                            Type::BoolType,
                            PrefixOperator::Bang,
                            Expression::BooleanLiteral(Rc::new(BooleanLiteral::<
                                TypedNodeCommonFields,
                            >::new(
                                true
                            )))
                        )))
                    )),
                )))
            ),])))
        );
    }

    #[test]
    fn prefix_expression_minus() {
        let sf_node = parse_and_check(test_inputs::PREFIX_OPERATION_MINUS);
        assert_eq!(
            sf_node,
            define_test_body(Rc::new(Block::new_with_final_expression(
                vec![],
                Expression::PrefixExpression(Rc::new(
                    PrefixExpression::<TypedNodeCommonFields>::new(
                        Type::I32Type,
                        PrefixOperator::Minus,
                        Expression::IntegerLiteral(Rc::new(
                            IntegerLiteral::<TypedNodeCommonFields>::new(4)
                        ))
                    )
                ))
            )))
        );
    }

    #[test]
    fn arithmetic_expressions() {
        let sf_node = parse_and_check(test_inputs::ARITHMETIC_OPERATOR_PRECEDENCE);
        assert_eq!(
            sf_node,
            define_test_body(Rc::new(Block::new_with_final_expression(
                vec![],
                Expression::BinaryExpression(Rc::new(
                    BinaryExpression::<TypedNodeCommonFields>::new(
                        Type::I32Type,
                        Expression::IntegerLiteral(Rc::new(
                            IntegerLiteral::<TypedNodeCommonFields>::new(2)
                        )),
                        InfixOperator::Plus,
                        Expression::BinaryExpression(Rc::new(BinaryExpression::<
                            TypedNodeCommonFields,
                        >::new(
                            Type::I32Type,
                            Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                TypedNodeCommonFields,
                            >::new(
                                2
                            ))),
                            InfixOperator::Times,
                            Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                TypedNodeCommonFields,
                            >::new(
                                2
                            ))),
                        ))),
                    )
                ))
            )))
        );
    }

    #[test]
    fn if_expressions() {
        let sf_node = parse_and_check(test_inputs::IF_EXPRESSION);
        assert_eq!(
            sf_node,
            define_test_body(Rc::new(Block::new_with_final_expression(
                vec![],
                Expression::IfExpression(Rc::new(IfExpression::<TypedNodeCommonFields>::new(
                    Type::I32Type,
                    Expression::BinaryExpression(Rc::new(
                        BinaryExpression::<TypedNodeCommonFields>::new(
                            Type::I32Type,
                            Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                TypedNodeCommonFields,
                            >::new(
                                1
                            ))),
                            InfixOperator::GreaterThan,
                            Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                TypedNodeCommonFields,
                            >::new(
                                2
                            )))
                        )
                    )),
                    Rc::new(Block::new_with_final_expression(
                        vec![],
                        Expression::IntegerLiteral(Rc::new(
                            IntegerLiteral::<TypedNodeCommonFields>::new(3)
                        ))
                    )),
                    Rc::new(Block::new_with_final_expression(
                        vec![],
                        Expression::IntegerLiteral(Rc::new(
                            IntegerLiteral::<TypedNodeCommonFields>::new(4)
                        ))
                    ))
                ))),
            )))
        )
    }

    #[test]
    fn function_calls() {
        let sf_node = parse_and_check(test_inputs::FUNCTION_CALL_ADD);
        assert_eq!(
            sf_node,
            Node::SourceFile(Rc::new(SourceFile::new(vec![
                Rc::new(Declaration::FunctionDeclaration(Rc::new(
                    FunctionDeclaration::<TypedNodeCommonFields>::new(
                        Type::UnitType,
                        Identifier::new("add".to_owned()),
                        Type::I32Type,
                        vec![
                            Rc::new(Parameter::<TypedNodeCommonFields>::new(
                                Type::I32Type,
                                Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                                    Type::I32Type,
                                    Identifier::new("a".to_owned()),
                                    Type::I32Type,
                                ))
                            )),
                            Rc::new(Parameter::<TypedNodeCommonFields>::new(
                                Type::I32Type,
                                Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                                    Type::I32Type,
                                    Identifier::new("b".to_owned()),
                                    Type::I32Type,
                                ))
                            ))
                        ],
                        Rc::new(Block::new_with_final_expression(
                            vec![],
                            Expression::BinaryExpression(Rc::new(BinaryExpression::<
                                TypedNodeCommonFields,
                            >::new(
                                Type::I32Type,
                                Expression::VariableReference(Rc::new(VariableReference::<
                                    TypedNodeCommonFields,
                                >::new(
                                    Type::I32Type,
                                    Identifier::new("a".to_owned())
                                ))),
                                InfixOperator::Plus,
                                Expression::VariableReference(Rc::new(VariableReference::<
                                    TypedNodeCommonFields,
                                >::new(
                                    Type::I32Type,
                                    Identifier::new("b".to_owned())
                                ))),
                            )))
                        ))
                    )
                ))),
                Rc::new(Declaration::FunctionDeclaration(Rc::new(
                    FunctionDeclaration::<TypedNodeCommonFields>::new(
                        Type::UnitType,
                        Identifier::new("test".to_owned(),),
                        Type::I32Type,
                        vec![],
                        Rc::new(Block::new_with_final_expression(
                            vec![],
                            Expression::FunctionCall(Rc::new(
                                FunctionCall::<TypedNodeCommonFields>::new(
                                    Type::I32Type,
                                    Identifier::new("add".to_owned()),
                                    vec![
                                        Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                            TypedNodeCommonFields,
                                        >::new(
                                            11
                                        ))),
                                        Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                            TypedNodeCommonFields,
                                        >::new(
                                            22
                                        )))
                                    ],
                                )
                            ))
                        ))
                    )
                ),)),
            ])))
        );
    }

    #[test]
    fn record_initializations_and_field_accesses() {
        let sf_node = parse_and_check(test_inputs::RECORD_INITIALIZATION_AND_FIELD_ACCESS);

        let b_type = Type::RecordType(Rc::new(RecordType::new(HashMap::from([(
            "c".to_string(),
            Type::I32Type,
        )]))));
        let x_type = Type::RecordType(Rc::new(RecordType::new(HashMap::from([
            ("a".to_string(), Type::BoolType),
            ("b".to_string(), b_type.clone()),
        ]))));

        assert_eq!(
            sf_node,
            define_test_body(Rc::new(Block::new_with_final_expression(
                vec![Statement::Declaration(Declaration::VariableDeclaration(
                    Rc::new(VariableDeclaration::<TypedNodeCommonFields>::new(
                        x_type.clone(),
                        Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                            x_type.clone(),
                            Identifier::new("x".to_owned()),
                            x_type.clone()
                        )),
                        Expression::RecordLiteral(Rc::new(
                            RecordLiteral::<TypedNodeCommonFields>::new(
                                x_type.clone(),
                                HashMap::from([
                                    (
                                        "a".to_string(),
                                        Expression::BooleanLiteral(Rc::new(BooleanLiteral::<
                                            TypedNodeCommonFields,
                                        >::new(
                                            true
                                        )))
                                    ),
                                    (
                                        "b".to_string(),
                                        Expression::RecordLiteral(Rc::new(RecordLiteral::<
                                            TypedNodeCommonFields,
                                        >::new(
                                            b_type.clone(),
                                            HashMap::from([(
                                                "c".to_string(),
                                                Expression::IntegerLiteral(Rc::new(
                                                    IntegerLiteral::<TypedNodeCommonFields>::new(
                                                        123
                                                    )
                                                ))
                                            )])
                                        )))
                                    )
                                ])
                            )
                        ))
                    ))
                ))],
                Expression::FieldAccess(Rc::new(FieldAccess::<TypedNodeCommonFields>::new(
                    Type::I32Type,
                    Expression::FieldAccess(Rc::new(FieldAccess::<TypedNodeCommonFields>::new(
                        b_type.clone(),
                        Expression::VariableReference(Rc::new(VariableReference::<
                            TypedNodeCommonFields,
                        >::new(
                            x_type.clone(),
                            Identifier::new("x".to_owned())
                        ))),
                        "b".to_string()
                    ))),
                    "c".to_string()
                )))
            )))
        )
    }

    #[test]
    fn variable_initializations_and_references() {
        let sf_node = parse_and_check(test_inputs::VARIABLE_INITIALIZATION_AND_REFERENCE_INT);
        assert_eq!(
            sf_node,
            define_test_body(Rc::new(Block::new_with_final_expression(
                vec![
                    Statement::Declaration(Declaration::VariableDeclaration(Rc::new(
                        VariableDeclaration::<TypedNodeCommonFields>::new(
                            Type::I32Type,
                            Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                                Type::I32Type,
                                Identifier::new("a".to_owned()),
                                Type::I32Type
                            )),
                            Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                TypedNodeCommonFields,
                            >::new(
                                10
                            )))
                        )
                    ))),
                    Statement::Declaration(Declaration::VariableDeclaration(Rc::new(
                        VariableDeclaration::<TypedNodeCommonFields>::new(
                            Type::I32Type,
                            Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                                Type::I32Type,
                                Identifier::new("b".to_owned()),
                                Type::I32Type
                            )),
                            Expression::VariableReference(Rc::new(VariableReference::<
                                TypedNodeCommonFields,
                            >::new(
                                Type::I32Type,
                                Identifier::new("a".to_owned())
                            )))
                        )
                    ))),
                ],
                Expression::VariableReference(Rc::new(
                    VariableReference::<TypedNodeCommonFields>::new(
                        Type::I32Type,
                        Identifier::new("b".to_owned())
                    )
                ))
            )))
        );
    }

    #[test]
    fn function_declarations() {
        let sf_node = parse_and_check(test_inputs::FUNCTION_DECLARATION_ADD);
        assert_eq!(
            sf_node,
            Node::SourceFile(Rc::new(SourceFile::new(vec![Rc::new(
                Declaration::FunctionDeclaration(Rc::new(FunctionDeclaration::<
                    TypedNodeCommonFields,
                >::new(
                    Type::UnitType,
                    Identifier::new("add".to_owned()),
                    Type::I32Type,
                    vec![
                        Rc::new(Parameter::<TypedNodeCommonFields>::new(
                            Type::I32Type,
                            Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                                Type::I32Type,
                                Identifier::new("a".to_owned()),
                                Type::I32Type,
                            ))
                        )),
                        Rc::new(Parameter::<TypedNodeCommonFields>::new(
                            Type::I32Type,
                            Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                                Type::I32Type,
                                Identifier::new("b".to_owned()),
                                Type::I32Type,
                            ))
                        ))
                    ],
                    Rc::new(Block::new_with_final_expression(
                        vec![],
                        Expression::BinaryExpression(Rc::new(BinaryExpression::<
                            TypedNodeCommonFields,
                        >::new(
                            Type::I32Type,
                            Expression::VariableReference(Rc::new(VariableReference::<
                                TypedNodeCommonFields,
                            >::new(
                                Type::I32Type,
                                Identifier::new("a".to_owned())
                            ))),
                            InfixOperator::Plus,
                            Expression::VariableReference(Rc::new(VariableReference::<
                                TypedNodeCommonFields,
                            >::new(
                                Type::I32Type,
                                Identifier::new("b".to_owned())
                            ))),
                        )))
                    ))
                )))
            )])))
        );
    }
}
