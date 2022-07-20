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
                    Type::TypeLiteral(TypeLiteral::UnitType),
                    Identifier::new("test".to_owned()),
                    Type::TypeLiteral(TypeLiteral::I32Type),
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
                    Type::TypeLiteral(TypeLiteral::UnitType),
                    Identifier::new("test".to_owned()),
                    Type::TypeLiteral(TypeLiteral::BoolType),
                    vec![],
                    Rc::new(Block::new_with_final_expression(
                        vec![],
                        Expression::PrefixExpression(Rc::new(PrefixExpression::<
                            TypedNodeCommonFields,
                        >::new(
                            Type::TypeLiteral(TypeLiteral::BoolType),
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
                        Type::TypeLiteral(TypeLiteral::I32Type),
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
                        Type::TypeLiteral(TypeLiteral::I32Type),
                        Expression::IntegerLiteral(Rc::new(
                            IntegerLiteral::<TypedNodeCommonFields>::new(2)
                        )),
                        InfixOperator::Plus,
                        Expression::BinaryExpression(Rc::new(BinaryExpression::<
                            TypedNodeCommonFields,
                        >::new(
                            Type::TypeLiteral(TypeLiteral::I32Type),
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
                    Type::TypeLiteral(TypeLiteral::I32Type),
                    Expression::BinaryExpression(Rc::new(
                        BinaryExpression::<TypedNodeCommonFields>::new(
                            Type::TypeLiteral(TypeLiteral::I32Type),
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
                        Type::TypeLiteral(TypeLiteral::UnitType),
                        Identifier::new("add".to_owned()),
                        Type::TypeLiteral(TypeLiteral::I32Type),
                        vec![
                            Rc::new(Parameter::<TypedNodeCommonFields>::new(
                                Type::TypeLiteral(TypeLiteral::I32Type),
                                Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                                    Type::TypeLiteral(TypeLiteral::I32Type),
                                    Identifier::new("a".to_owned()),
                                    Type::TypeLiteral(TypeLiteral::I32Type),
                                ))
                            )),
                            Rc::new(Parameter::<TypedNodeCommonFields>::new(
                                Type::TypeLiteral(TypeLiteral::I32Type),
                                Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                                    Type::TypeLiteral(TypeLiteral::I32Type),
                                    Identifier::new("b".to_owned()),
                                    Type::TypeLiteral(TypeLiteral::I32Type),
                                ))
                            ))
                        ],
                        Rc::new(Block::new_with_final_expression(
                            vec![],
                            Expression::BinaryExpression(Rc::new(BinaryExpression::<
                                TypedNodeCommonFields,
                            >::new(
                                Type::TypeLiteral(TypeLiteral::I32Type),
                                Expression::VariableReference(Rc::new(VariableReference::<
                                    TypedNodeCommonFields,
                                >::new(
                                    Type::TypeLiteral(TypeLiteral::I32Type),
                                    Identifier::new("a".to_owned())
                                ))),
                                InfixOperator::Plus,
                                Expression::VariableReference(Rc::new(VariableReference::<
                                    TypedNodeCommonFields,
                                >::new(
                                    Type::TypeLiteral(TypeLiteral::I32Type),
                                    Identifier::new("b".to_owned())
                                ))),
                            )))
                        ))
                    )
                ))),
                Rc::new(Declaration::FunctionDeclaration(Rc::new(
                    FunctionDeclaration::<TypedNodeCommonFields>::new(
                        Type::TypeLiteral(TypeLiteral::UnitType),
                        Identifier::new("test".to_owned(),),
                        Type::TypeLiteral(TypeLiteral::I32Type),
                        vec![],
                        Rc::new(Block::new_with_final_expression(
                            vec![],
                            Expression::FunctionCall(Rc::new(
                                FunctionCall::<TypedNodeCommonFields>::new(
                                    Type::TypeLiteral(TypeLiteral::I32Type),
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
    fn variable_initializations_and_references() {
        let sf_node = parse_and_check(test_inputs::VARIABLE_INITIALIZATION_AND_REFERENCE_INT);
        assert_eq!(
            sf_node,
            define_test_body(Rc::new(Block::new_with_final_expression(
                vec![
                    Statement::Declaration(Declaration::VariableDeclaration(Rc::new(
                        VariableDeclaration::<TypedNodeCommonFields>::new(
                            Type::TypeLiteral(TypeLiteral::I32Type),
                            Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                                Type::TypeLiteral(TypeLiteral::I32Type),
                                Identifier::new("a".to_owned()),
                                Type::TypeLiteral(TypeLiteral::I32Type)
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
                            Type::TypeLiteral(TypeLiteral::I32Type),
                            Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                                Type::TypeLiteral(TypeLiteral::I32Type),
                                Identifier::new("b".to_owned()),
                                Type::TypeLiteral(TypeLiteral::I32Type)
                            )),
                            Expression::VariableReference(Rc::new(VariableReference::<
                                TypedNodeCommonFields,
                            >::new(
                                Type::TypeLiteral(TypeLiteral::I32Type),
                                Identifier::new("a".to_owned())
                            )))
                        )
                    ))),
                ],
                Expression::VariableReference(Rc::new(
                    VariableReference::<TypedNodeCommonFields>::new(
                        Type::TypeLiteral(TypeLiteral::I32Type),
                        Identifier::new("b".to_owned())
                    )
                ))
            )))
        );
    }

    #[test]
    fn variable_initialization_record_type() {
        let sf_node = parse_and_check(test_inputs::VARIABLE_INITIALIZATION_RECORD_TYPE);
        let record_type = Type::RecordType(Rc::new(RecordType::new(HashMap::from([
            ("a".to_string(), Type::TypeLiteral(TypeLiteral::I32Type)),
            ("b".to_string(), Type::TypeLiteral(TypeLiteral::BoolType)),
        ]))));
        assert_eq!(
            sf_node,
            define_test_body(Rc::new(Block::new_with_final_expression(
                vec![Statement::Declaration(Declaration::VariableDeclaration(
                    Rc::new(VariableDeclaration::<TypedNodeCommonFields>::new(
                        record_type.clone(),
                        Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                            record_type.clone(),
                            Identifier::new("x".to_owned()),
                            record_type.clone()
                        )),
                        Expression::RecordLiteral(Rc::new(
                            RecordLiteral::<TypedNodeCommonFields>::new(
                                record_type.clone(),
                                HashMap::from([
                                    (
                                        "a".to_string(),
                                        Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                            TypedNodeCommonFields,
                                        >::new(
                                            11
                                        )))
                                    ),
                                    (
                                        "b".to_string(),
                                        Expression::BooleanLiteral(Rc::new(BooleanLiteral::<
                                            TypedNodeCommonFields,
                                        >::new(
                                            true
                                        )))
                                    )
                                ])
                            )
                        ))
                    ))
                ))],
                Expression::IntegerLiteral(Rc::new(IntegerLiteral::<TypedNodeCommonFields>::new(
                    0
                )))
            )))
        )
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
                    Type::TypeLiteral(TypeLiteral::UnitType),
                    Identifier::new("add".to_owned()),
                    Type::TypeLiteral(TypeLiteral::I32Type),
                    vec![
                        Rc::new(Parameter::<TypedNodeCommonFields>::new(
                            Type::TypeLiteral(TypeLiteral::I32Type),
                            Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                                Type::TypeLiteral(TypeLiteral::I32Type),
                                Identifier::new("a".to_owned()),
                                Type::TypeLiteral(TypeLiteral::I32Type),
                            ))
                        )),
                        Rc::new(Parameter::<TypedNodeCommonFields>::new(
                            Type::TypeLiteral(TypeLiteral::I32Type),
                            Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                                Type::TypeLiteral(TypeLiteral::I32Type),
                                Identifier::new("b".to_owned()),
                                Type::TypeLiteral(TypeLiteral::I32Type),
                            ))
                        ))
                    ],
                    Rc::new(Block::new_with_final_expression(
                        vec![],
                        Expression::BinaryExpression(Rc::new(BinaryExpression::<
                            TypedNodeCommonFields,
                        >::new(
                            Type::TypeLiteral(TypeLiteral::I32Type),
                            Expression::VariableReference(Rc::new(VariableReference::<
                                TypedNodeCommonFields,
                            >::new(
                                Type::TypeLiteral(TypeLiteral::I32Type),
                                Identifier::new("a".to_owned())
                            ))),
                            InfixOperator::Plus,
                            Expression::VariableReference(Rc::new(VariableReference::<
                                TypedNodeCommonFields,
                            >::new(
                                Type::TypeLiteral(TypeLiteral::I32Type),
                                Identifier::new("b".to_owned())
                            ))),
                        )))
                    ))
                )))
            )])))
        );
    }
}
