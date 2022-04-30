use crate::*;

mod tests {
    use super::*;

    fn define_test_body(body: Rc<Block<UntypedNodeCommonFields>>) -> Node<UntypedNodeCommonFields> {
        Node::SourceFile(Rc::new(SourceFile::new(vec![Rc::new(
            Declaration::FunctionDeclaration(Rc::new(
                FunctionDeclaration::<UntypedNodeCommonFields>::new(
                    Identifier::new("test".to_owned()),
                    Type::TypeLiteral(TypeLiteral::I32Type),
                    vec![],
                    body,
                ),
            )),
        )])))
    }

    fn parse(source: &str) -> Node<UntypedNodeCommonFields> {
        let mut parser = Parser::new(source);
        let output = parser.parse_source_file();
        Node::SourceFile(Rc::new(output))
    }

    #[test]
    fn prefix_expression_not() {
        let sf_node = parse(test_inputs::PREFIX_OPERATION_NOT);
        assert_eq!(
            sf_node,
            Node::SourceFile(Rc::new(SourceFile::new(vec![Rc::new(
                Declaration::FunctionDeclaration(Rc::new(FunctionDeclaration::<
                    UntypedNodeCommonFields,
                >::new(
                    Identifier::new("test".to_owned()),
                    Type::TypeLiteral(TypeLiteral::BoolType),
                    vec![],
                    Rc::new(Block::new_with_final_expression(
                        vec![],
                        Expression::PrefixExpression(Rc::new(PrefixExpression::<
                            UntypedNodeCommonFields,
                        >::new(
                            PrefixOperator::Bang,
                            Expression::BooleanLiteral(Rc::new(BooleanLiteral::<
                                UntypedNodeCommonFields,
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
        let sf_node = parse(test_inputs::PREFIX_OPERATION_MINUS);
        assert_eq!(
            sf_node,
            define_test_body(Rc::new(Block::new_with_final_expression(
                vec![],
                Expression::PrefixExpression(Rc::new(
                    PrefixExpression::<UntypedNodeCommonFields>::new(
                        PrefixOperator::Minus,
                        Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                            UntypedNodeCommonFields,
                        >::new(4)))
                    )
                ))
            )))
        );
    }

    #[test]
    fn arithmetic_expressions() {
        let sf_node = parse(test_inputs::ARITHMETIC_OPERATOR_PRECEDENCE);
        assert_eq!(
            sf_node,
            define_test_body(Rc::new(Block::new_with_final_expression(
                vec![],
                Expression::BinaryExpression(Rc::new(
                    BinaryExpression::<UntypedNodeCommonFields>::new(
                        Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                            UntypedNodeCommonFields,
                        >::new(2))),
                        InfixOperator::Plus,
                        Expression::BinaryExpression(Rc::new(BinaryExpression::<
                            UntypedNodeCommonFields,
                        >::new(
                            Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                UntypedNodeCommonFields,
                            >::new(
                                2
                            ))),
                            InfixOperator::Times,
                            Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                UntypedNodeCommonFields,
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
        let sf_node = parse(test_inputs::IF_EXPRESSION);
        assert_eq!(
            sf_node,
            define_test_body(Rc::new(Block::new_with_final_expression(
                vec![],
                Expression::IfExpression(Rc::new(IfExpression::<UntypedNodeCommonFields>::new(
                    Expression::BinaryExpression(Rc::new(BinaryExpression::<
                        UntypedNodeCommonFields,
                    >::new(
                        Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                            UntypedNodeCommonFields,
                        >::new(1))),
                        InfixOperator::GreaterThan,
                        Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                            UntypedNodeCommonFields,
                        >::new(2)))
                    ))),
                    Rc::new(Block::new_with_final_expression(
                        vec![],
                        Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                            UntypedNodeCommonFields,
                        >::new(3)))
                    )),
                    Rc::new(Block::new_with_final_expression(
                        vec![],
                        Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                            UntypedNodeCommonFields,
                        >::new(4)))
                    ))
                )))
            )))
        )
    }

    #[test]
    fn function_call_add() {
        let sf_node = parse(test_inputs::FUNCTION_CALL_ADD);
        assert_eq!(
            sf_node,
            Node::SourceFile(Rc::new(SourceFile::new(vec![
                Rc::new(Declaration::FunctionDeclaration(Rc::new(
                    FunctionDeclaration::<UntypedNodeCommonFields>::new(
                        Identifier::new("add".to_owned()),
                        Type::TypeLiteral(TypeLiteral::I32Type),
                        vec![
                            Rc::new(Parameter::<UntypedNodeCommonFields>::new(Rc::new(
                                VariableDeclarator::<UntypedNodeCommonFields>::new(
                                    Identifier::new("a".to_owned()),
                                    Type::TypeLiteral(TypeLiteral::I32Type),
                                )
                            ))),
                            Rc::new(Parameter::<UntypedNodeCommonFields>::new(Rc::new(
                                VariableDeclarator::<UntypedNodeCommonFields>::new(
                                    Identifier::new("b".to_owned()),
                                    Type::TypeLiteral(TypeLiteral::I32Type),
                                )
                            )))
                        ],
                        Rc::new(Block::new_with_final_expression(
                            vec![],
                            Expression::BinaryExpression(Rc::new(BinaryExpression::<
                                UntypedNodeCommonFields,
                            >::new(
                                Expression::VariableReference(Rc::new(VariableReference::<
                                    UntypedNodeCommonFields,
                                >::new(
                                    Identifier::new("a".to_owned())
                                ))),
                                InfixOperator::Plus,
                                Expression::VariableReference(Rc::new(VariableReference::<
                                    UntypedNodeCommonFields,
                                >::new(
                                    Identifier::new("b".to_owned())
                                ))),
                            )))
                        ))
                    )
                ))),
                Rc::new(Declaration::FunctionDeclaration(Rc::new(
                    FunctionDeclaration::<UntypedNodeCommonFields>::new(
                        Identifier::new("test".to_owned(),),
                        Type::TypeLiteral(TypeLiteral::I32Type),
                        vec![],
                        Rc::new(Block::new_with_final_expression(
                            vec![],
                            Expression::FunctionCall(Rc::new(FunctionCall::<
                                UntypedNodeCommonFields,
                            >::new(
                                Identifier::new("add".to_owned()),
                                vec![
                                    Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                        UntypedNodeCommonFields,
                                    >::new(
                                        11
                                    ))),
                                    Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                        UntypedNodeCommonFields,
                                    >::new(
                                        22
                                    )))
                                ],
                            )))
                        ))
                    )
                ),)),
            ])))
        );
    }

    #[test]
    fn variable_references() {
        let sf_node = parse(test_inputs::VARIABLE_REFERENCE);
        assert_eq!(
            sf_node,
            define_test_body(Rc::new(Block::new_with_final_expression(
                vec![
                    Statement::Declaration(Declaration::VariableDeclaration(Rc::new(
                        VariableDeclaration::<UntypedNodeCommonFields>::new(
                            Rc::new(VariableDeclarator::<UntypedNodeCommonFields>::new(
                                Identifier::new("a".to_owned()),
                                Type::TypeLiteral(TypeLiteral::I32Type)
                            )),
                            Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                UntypedNodeCommonFields,
                            >::new(
                                10
                            )))
                        )
                    ))),
                    Statement::Declaration(Declaration::VariableDeclaration(Rc::new(
                        VariableDeclaration::<UntypedNodeCommonFields>::new(
                            Rc::new(VariableDeclarator::<UntypedNodeCommonFields>::new(
                                Identifier::new("b".to_owned()),
                                Type::TypeLiteral(TypeLiteral::I32Type)
                            )),
                            Expression::VariableReference(Rc::new(VariableReference::<
                                UntypedNodeCommonFields,
                            >::new(
                                Identifier::new("a".to_owned())
                            )))
                        )
                    ))),
                ],
                Expression::VariableReference(Rc::new(
                    VariableReference::<UntypedNodeCommonFields>::new(Identifier::new(
                        "b".to_owned()
                    ))
                ))
            )))
        );
    }

    #[test]
    fn variable_initialization_int() {
        let sf_node = parse(test_inputs::VARIABLE_INITIALIZATION_INT);
        assert_eq!(
            sf_node,
            define_test_body(Rc::new(Block::new_with_final_expression(
                vec![Statement::Declaration(Declaration::VariableDeclaration(
                    Rc::new(VariableDeclaration::<UntypedNodeCommonFields>::new(
                        Rc::new(VariableDeclarator::<UntypedNodeCommonFields>::new(
                            Identifier::new("x".to_owned()),
                            Type::TypeLiteral(TypeLiteral::I32Type)
                        )),
                        Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                            UntypedNodeCommonFields,
                        >::new(1)))
                    ))
                ))],
                Expression::VariableReference(Rc::new(
                    VariableReference::<UntypedNodeCommonFields>::new(Identifier::new(
                        "x".to_owned()
                    ))
                ))
            )))
        )
    }

    #[test]
    fn function_declarations() {
        let sf_node = parse(test_inputs::FUNCTION_DECLARATION_ADD);
        assert_eq!(
            sf_node,
            Node::SourceFile(Rc::new(SourceFile::new(vec![Rc::new(
                Declaration::FunctionDeclaration(Rc::new(FunctionDeclaration::<
                    UntypedNodeCommonFields,
                >::new(
                    Identifier::new("add".to_owned()),
                    Type::TypeLiteral(TypeLiteral::I32Type),
                    vec![
                        Rc::new(Parameter::<UntypedNodeCommonFields>::new(Rc::new(
                            VariableDeclarator::<UntypedNodeCommonFields>::new(
                                Identifier::new("a".to_owned()),
                                Type::TypeLiteral(TypeLiteral::I32Type),
                            )
                        ))),
                        Rc::new(Parameter::<UntypedNodeCommonFields>::new(Rc::new(
                            VariableDeclarator::<UntypedNodeCommonFields>::new(
                                Identifier::new("b".to_owned()),
                                Type::TypeLiteral(TypeLiteral::I32Type),
                            )
                        )))
                    ],
                    Rc::new(Block::new_with_final_expression(
                        vec![],
                        Expression::BinaryExpression(Rc::new(BinaryExpression::<
                            UntypedNodeCommonFields,
                        >::new(
                            Expression::VariableReference(Rc::new(VariableReference::<
                                UntypedNodeCommonFields,
                            >::new(
                                Identifier::new("a".to_owned())
                            ))),
                            InfixOperator::Plus,
                            Expression::VariableReference(Rc::new(VariableReference::<
                                UntypedNodeCommonFields,
                            >::new(
                                Identifier::new("b".to_owned())
                            ))),
                        )))
                    ))
                )))
            )])))
        );
    }
}
