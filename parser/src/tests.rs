use crate::*;

mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn define_test_body(body: Rc<Block<UntypedNodeCommonFields>>) -> Node<UntypedNodeCommonFields> {
        Node::SourceFile(Rc::new(SourceFile::new(
            vec![Declaration::FunctionDeclaration(Rc::new(
                FunctionDeclaration::<UntypedNodeCommonFields>::new(
                    Identifier::new("test".to_owned()),
                    Type::I64Type,
                    vec![],
                    body,
                ),
            ))],
            vec![],
        )))
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
            Node::SourceFile(Rc::new(SourceFile::new(
                vec![Declaration::FunctionDeclaration(Rc::new(
                    FunctionDeclaration::<UntypedNodeCommonFields>::new(
                        Identifier::new("test".to_owned()),
                        Type::BoolType,
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
                    )
                )),],
                vec![]
            )))
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
    fn object_initialization_with_type_arguments_and_field_access() {
        let sf_node =
            parse(test_inputs::OBJECT_INITIALIZATION_WITH_TYPE_ARGUMENTS_AND_FIELD_ACCESS);
        assert_eq!(
            sf_node,
            Node::SourceFile(Rc::new(SourceFile::new(
                vec![Declaration::FunctionDeclaration(Rc::new(
                    FunctionDeclaration::<UntypedNodeCommonFields>::new(
                        Identifier::new("test".to_owned()),
                        Type::I64Type,
                        vec![],
                        Rc::new(Block::new_with_final_expression(
                            vec![Statement::Declaration(Declaration::VariableDeclaration(
                                Rc::new(VariableDeclaration::<UntypedNodeCommonFields>::new(
                                    Rc::new(VariableDeclarator::<UntypedNodeCommonFields>::new(
                                        Identifier::new("y".to_owned()),
                                        Type::TypeReference(Rc::new(TypeReference::new(
                                            Identifier::new("Y".to_owned()),
                                            vec![Type::I64Type]
                                        )))
                                    )),
                                    Expression::ObjectLiteral(Rc::new(ObjectLiteral::<
                                        UntypedNodeCommonFields,
                                    >::new(
                                        TypeReference::new(
                                            Identifier::new("Y".to_owned()),
                                            vec![Type::I64Type]
                                        ),
                                        HashMap::from([
                                            (
                                                "a".to_string(),
                                                Expression::BooleanLiteral(Rc::new(
                                                    BooleanLiteral::<UntypedNodeCommonFields>::new(
                                                        true
                                                    )
                                                ))
                                            ),
                                            (
                                                "b".to_string(),
                                                Expression::ObjectLiteral(Rc::new(
                                                    ObjectLiteral::<UntypedNodeCommonFields>::new(
                                                        TypeReference::new(
                                                            Identifier::new("X".to_owned()),
                                                            vec![Type::I64Type]
                                                        ),
                                                        HashMap::from([(
                                                            "c".to_string(),
                                                            Expression::IntegerLiteral(Rc::new(
                                                                IntegerLiteral::<
                                                                    UntypedNodeCommonFields,
                                                                >::new(
                                                                    123
                                                                )
                                                            ))
                                                        )])
                                                    )
                                                ))
                                            )
                                        ])
                                    )))
                                ))
                            ))],
                            Expression::FieldAccess(Rc::new(
                                FieldAccess::<UntypedNodeCommonFields>::new(
                                    Expression::FieldAccess(Rc::new(FieldAccess::<
                                        UntypedNodeCommonFields,
                                    >::new(
                                        Expression::VariableReference(Rc::new(
                                            VariableReference::<UntypedNodeCommonFields>::new(
                                                Identifier::new("y".to_owned())
                                            )
                                        )),
                                        "b".to_string()
                                    ))),
                                    "c".to_string()
                                )
                            ))
                        )),
                    ),
                ),)],
                vec![
                    ClassDeclaration::new(
                        Identifier::new("X".to_owned()),
                        vec![Identifier::new("T".to_owned())],
                        HashMap::from([(
                            "c".to_string(),
                            Type::TypeReference(Rc::new(TypeReference::new(
                                Identifier::new("T".to_owned()),
                                vec![]
                            )))
                        )]),
                    ),
                    ClassDeclaration::new(
                        Identifier::new("Y".to_owned()),
                        vec![Identifier::new("T".to_owned())],
                        HashMap::from([
                            ("a".to_string(), Type::BoolType),
                            (
                                "b".to_string(),
                                Type::TypeReference(Rc::new(TypeReference::new(
                                    Identifier::new("X".to_owned()),
                                    vec![Type::TypeReference(Rc::new(TypeReference::new(
                                        Identifier::new("T".to_owned()),
                                        vec![]
                                    )))]
                                ))),
                            )
                        ]),
                    ),
                ]
            )))
        )
    }

    #[test]
    fn object_initialization_and_field_access() {
        let sf_node = parse(test_inputs::OBJECT_INITIALIZATION_AND_FIELD_ACCESS);
        assert_eq!(
            sf_node,
            Node::SourceFile(Rc::new(SourceFile::new(
                vec![Declaration::FunctionDeclaration(Rc::new(
                    FunctionDeclaration::<UntypedNodeCommonFields>::new(
                        Identifier::new("test".to_owned()),
                        Type::I64Type,
                        vec![],
                        Rc::new(Block::new_with_final_expression(
                            vec![Statement::Declaration(Declaration::VariableDeclaration(
                                Rc::new(VariableDeclaration::<UntypedNodeCommonFields>::new(
                                    Rc::new(VariableDeclarator::<UntypedNodeCommonFields>::new(
                                        Identifier::new("y".to_owned()),
                                        Type::TypeReference(Rc::new(TypeReference::new(
                                            Identifier::new("Y".to_owned()),
                                            vec![]
                                        )))
                                    )),
                                    Expression::ObjectLiteral(Rc::new(ObjectLiteral::<
                                        UntypedNodeCommonFields,
                                    >::new(
                                        TypeReference::new(Identifier::new("Y".to_owned()), vec![]),
                                        HashMap::from([
                                            (
                                                "a".to_string(),
                                                Expression::BooleanLiteral(Rc::new(
                                                    BooleanLiteral::<UntypedNodeCommonFields>::new(
                                                        true
                                                    )
                                                ))
                                            ),
                                            (
                                                "b".to_string(),
                                                Expression::ObjectLiteral(Rc::new(
                                                    ObjectLiteral::<UntypedNodeCommonFields>::new(
                                                        TypeReference::new(
                                                            Identifier::new("X".to_owned()),
                                                            vec![]
                                                        ),
                                                        HashMap::from([(
                                                            "c".to_string(),
                                                            Expression::IntegerLiteral(Rc::new(
                                                                IntegerLiteral::<
                                                                    UntypedNodeCommonFields,
                                                                >::new(
                                                                    123
                                                                )
                                                            ))
                                                        )])
                                                    )
                                                ))
                                            )
                                        ])
                                    )))
                                ))
                            ))],
                            Expression::FieldAccess(Rc::new(
                                FieldAccess::<UntypedNodeCommonFields>::new(
                                    Expression::FieldAccess(Rc::new(FieldAccess::<
                                        UntypedNodeCommonFields,
                                    >::new(
                                        Expression::VariableReference(Rc::new(
                                            VariableReference::<UntypedNodeCommonFields>::new(
                                                Identifier::new("y".to_owned())
                                            )
                                        )),
                                        "b".to_string()
                                    ))),
                                    "c".to_string()
                                )
                            ))
                        )),
                    ),
                ),)],
                vec![
                    ClassDeclaration::new(
                        Identifier::new("X".to_owned()),
                        vec![],
                        HashMap::from([("c".to_string(), Type::I64Type)]),
                    ),
                    ClassDeclaration::new(
                        Identifier::new("Y".to_owned()),
                        vec![],
                        HashMap::from([
                            ("a".to_string(), Type::BoolType),
                            (
                                "b".to_string(),
                                Type::TypeReference(Rc::new(TypeReference::new(
                                    Identifier::new("X".to_owned()),
                                    vec![]
                                ))),
                            )
                        ]),
                    ),
                ]
            )))
        )
    }

    #[test]
    fn variable_initializations_and_references() {
        let sf_node = parse(test_inputs::VARIABLE_INITIALIZATION_AND_REFERENCE_INT);
        assert_eq!(
            sf_node,
            define_test_body(Rc::new(Block::new_with_final_expression(
                vec![
                    Statement::Declaration(Declaration::VariableDeclaration(Rc::new(
                        VariableDeclaration::<UntypedNodeCommonFields>::new(
                            Rc::new(VariableDeclarator::<UntypedNodeCommonFields>::new(
                                Identifier::new("a".to_owned()),
                                Type::I64Type
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
                                Type::I64Type
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
    fn function_declarations() {
        let sf_node = parse(test_inputs::FUNCTION_DECLARATION_ADD);
        assert_eq!(
            sf_node,
            Node::SourceFile(Rc::new(SourceFile::new(
                vec![Declaration::FunctionDeclaration(Rc::new(
                    FunctionDeclaration::<UntypedNodeCommonFields>::new(
                        Identifier::new("add".to_owned()),
                        Type::I64Type,
                        vec![
                            Rc::new(Parameter::<UntypedNodeCommonFields>::new(Rc::new(
                                VariableDeclarator::<UntypedNodeCommonFields>::new(
                                    Identifier::new("a".to_owned()),
                                    Type::I64Type,
                                )
                            ))),
                            Rc::new(Parameter::<UntypedNodeCommonFields>::new(Rc::new(
                                VariableDeclarator::<UntypedNodeCommonFields>::new(
                                    Identifier::new("b".to_owned()),
                                    Type::I64Type,
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
                ))],
                vec![]
            )))
        );
    }

    #[test]
    fn function_call_add() {
        let sf_node = parse(test_inputs::FUNCTION_CALL_ADD);
        assert_eq!(
            sf_node,
            Node::SourceFile(Rc::new(SourceFile::new(
                vec![
                    Declaration::FunctionDeclaration(Rc::new(FunctionDeclaration::<
                        UntypedNodeCommonFields,
                    >::new(
                        Identifier::new("add".to_owned()),
                        Type::I64Type,
                        vec![
                            Rc::new(Parameter::<UntypedNodeCommonFields>::new(Rc::new(
                                VariableDeclarator::<UntypedNodeCommonFields>::new(
                                    Identifier::new("a".to_owned()),
                                    Type::I64Type,
                                )
                            ))),
                            Rc::new(Parameter::<UntypedNodeCommonFields>::new(Rc::new(
                                VariableDeclarator::<UntypedNodeCommonFields>::new(
                                    Identifier::new("b".to_owned()),
                                    Type::I64Type,
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
                    ))),
                    Declaration::FunctionDeclaration(Rc::new(FunctionDeclaration::<
                        UntypedNodeCommonFields,
                    >::new(
                        Identifier::new("test".to_owned(),),
                        Type::I64Type,
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
                    )),),
                ],
                vec![]
            )))
        );
    }

    #[test]
    fn function_call_update_state() {
        let sf_node = parse(test_inputs::FUNCTION_CALL_UPDATE_STATE);

        let c_dot_a_field_access = Expression::FieldAccess(Rc::new(FieldAccess::<
            UntypedNodeCommonFields,
        >::new(
            Expression::VariableReference(Rc::new(
                VariableReference::<UntypedNodeCommonFields>::new(Identifier::new("c".to_owned())),
            )),
            "a".to_string(),
        )));

        assert_eq!(
            sf_node,
            Node::SourceFile(Rc::new(SourceFile::new(
                vec![
                    Declaration::FunctionDeclaration(Rc::new(FunctionDeclaration::<
                        UntypedNodeCommonFields,
                    >::new(
                        Identifier::new("updateState".to_owned()),
                        Type::TypeReference(Rc::new(TypeReference::new(
                            Identifier::new("C".to_owned()),
                            vec![Type::I64Type]
                        ))),
                        vec![Rc::new(Parameter::<UntypedNodeCommonFields>::new(Rc::new(
                            VariableDeclarator::<UntypedNodeCommonFields>::new(
                                Identifier::new("c".to_owned()),
                                Type::TypeReference(Rc::new(TypeReference::new(
                                    Identifier::new("C".to_owned()),
                                    vec![Type::I64Type]
                                )))
                            )
                        )))],
                        Rc::new(Block::new_with_final_expression(
                            vec![],
                            Expression::ObjectLiteral(Rc::new(ObjectLiteral::<
                                UntypedNodeCommonFields,
                            >::new(
                                TypeReference::new(
                                    Identifier::new("C".to_owned()),
                                    vec![Type::I64Type]
                                ),
                                HashMap::from([(
                                    "a".to_string(),
                                    Expression::BinaryExpression(Rc::new(BinaryExpression::<
                                        UntypedNodeCommonFields,
                                    >::new(
                                        c_dot_a_field_access,
                                        InfixOperator::Plus,
                                        Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                            UntypedNodeCommonFields,
                                        >::new(
                                            1
                                        )))
                                    )))
                                ),])
                            )))
                        ))
                    ))),
                    Declaration::FunctionDeclaration(Rc::new(FunctionDeclaration::<
                        UntypedNodeCommonFields,
                    >::new(
                        Identifier::new("test".to_owned(),),
                        Type::I64Type,
                        vec![],
                        Rc::new(Block::new_with_final_expression(
                            vec![Statement::Declaration(Declaration::VariableDeclaration(
                                Rc::new(VariableDeclaration::<UntypedNodeCommonFields>::new(
                                    Rc::new(VariableDeclarator::<UntypedNodeCommonFields>::new(
                                        Identifier::new("c".to_owned()),
                                        Type::TypeReference(Rc::new(TypeReference::new(
                                            Identifier::new("C".to_owned()),
                                            vec![Type::I64Type]
                                        )))
                                    )),
                                    Expression::ObjectLiteral(Rc::new(ObjectLiteral::<
                                        UntypedNodeCommonFields,
                                    >::new(
                                        TypeReference::new(
                                            Identifier::new("C".to_owned()),
                                            vec![Type::I64Type]
                                        ),
                                        HashMap::from([(
                                            "a".to_string(),
                                            Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                                UntypedNodeCommonFields,
                                            >::new(
                                                0
                                            )))
                                        ),])
                                    )))
                                ))
                            )),],
                            Expression::FieldAccess(Rc::new(
                                FieldAccess::<UntypedNodeCommonFields>::new(
                                    Expression::FunctionCall(Rc::new(FunctionCall::<
                                        UntypedNodeCommonFields,
                                    >::new(
                                        Identifier::new("updateState".to_owned()),
                                        vec![Expression::VariableReference(Rc::new(
                                            VariableReference::<UntypedNodeCommonFields>::new(
                                                Identifier::new("c".to_owned()),
                                            ),
                                        ))],
                                    ))),
                                    "a".to_string(),
                                )
                            )),
                        ))
                    )),),
                ],
                vec![ClassDeclaration::new(
                    Identifier::new("C".to_owned()),
                    vec![Identifier::new("T".to_owned())],
                    HashMap::from([(
                        "a".to_string(),
                        Type::TypeReference(Rc::new(TypeReference::new(
                            Identifier::new("T".to_owned()),
                            vec![]
                        )))
                    )]),
                )]
            )))
        );
    }
}
