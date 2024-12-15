use super::type_checker::TypeChecker;
use crate::visitor::PostOrderVisitor;
use dishsoap_parser::ast::*;
use dishsoap_parser::test_inputs;
use dishsoap_parser::Parser;
use std::collections::HashMap;
use std::rc::Rc;

mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn define_test_body(body: Rc<Block<TypedNodeCommonFields>>) -> Node<TypedNodeCommonFields> {
        Node::SourceFile(Rc::new(SourceFile::new(
            vec![Declaration::FunctionDeclaration(Rc::new(
                FunctionDeclaration::<TypedNodeCommonFields>::new(
                    Type::FunctionType(Rc::new(FunctionType::new(vec![], Type::I32Type))),
                    Identifier::new("test".to_owned()),
                    Type::I32Type,
                    vec![],
                    body,
                ),
            ))],
            vec![],
        )))
    }

    fn parse_and_check(source: &str) -> Node<TypedNodeCommonFields> {
        let mut parser = Parser::new(source);
        let untyped_ast = parser.parse();
        let mut type_checker = TypeChecker::new(&untyped_ast);
        let typed_ast = type_checker.visit(&untyped_ast).clone();

        typed_ast
    }

    #[test]
    fn prefix_expression_not() {
        let sf_node = parse_and_check(test_inputs::PREFIX_OPERATION_NOT);
        assert_eq!(
            sf_node,
            Node::SourceFile(Rc::new(SourceFile::new(
                vec![Declaration::FunctionDeclaration(Rc::new(
                    FunctionDeclaration::<TypedNodeCommonFields>::new(
                        Type::FunctionType(Rc::new(FunctionType::new(vec![], Type::BoolType))),
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
                    )
                )),],
                vec![]
            )))
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
    fn object_initialization_with_type_arguments_and_field_access() {
        let sf_node = parse_and_check(
            test_inputs::OBJECT_INITIALIZATION_WITH_TYPE_ARGUMENTS_AND_FIELD_ACCESS,
        );

        let x_type = Type::RecordType(Rc::new(RecordType::new(HashMap::from([(
            "c".to_string(),
            Type::I32Type,
        )]))));
        let y_type = Type::RecordType(Rc::new(RecordType::new(HashMap::from([
            ("a".to_string(), Type::BoolType),
            ("b".to_string(), x_type.clone()),
        ]))));

        assert_eq!(
            sf_node,
            Node::SourceFile(Rc::new(SourceFile::new(
                vec![Declaration::FunctionDeclaration(Rc::new(
                    FunctionDeclaration::<TypedNodeCommonFields>::new(
                        Type::FunctionType(Rc::new(FunctionType::new(vec![], Type::I32Type))),
                        Identifier::new("test".to_owned()),
                        Type::I32Type,
                        vec![],
                        Rc::new(Block::new_with_final_expression(
                            vec![Statement::Declaration(Declaration::VariableDeclaration(
                                Rc::new(VariableDeclaration::<TypedNodeCommonFields>::new(
                                    y_type.clone(),
                                    Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                                        y_type.clone(),
                                        Identifier::new("y".to_owned()),
                                        Type::TypeReference(Rc::new(TypeReference::new(
                                            Identifier::new("Y".to_owned()),
                                            vec![Type::I32Type]
                                        )))
                                    )),
                                    Expression::ObjectLiteral(Rc::new(ObjectLiteral::<
                                        TypedNodeCommonFields,
                                    >::new(
                                        y_type.clone(),
                                        TypeReference::new(
                                            Identifier::new("Y".to_owned()),
                                            vec![Type::I32Type]
                                        ),
                                        HashMap::from([
                                            (
                                                "a".to_string(),
                                                Expression::BooleanLiteral(Rc::new(
                                                    BooleanLiteral::<TypedNodeCommonFields>::new(
                                                        true
                                                    )
                                                ))
                                            ),
                                            (
                                                "b".to_string(),
                                                Expression::ObjectLiteral(Rc::new(
                                                    ObjectLiteral::<TypedNodeCommonFields>::new(
                                                        x_type.clone(),
                                                        TypeReference::new(
                                                            Identifier::new("X".to_owned()),
                                                            vec![Type::I32Type]
                                                        ),
                                                        HashMap::from([(
                                                            "c".to_string(),
                                                            Expression::IntegerLiteral(Rc::new(
                                                                IntegerLiteral::<
                                                                    TypedNodeCommonFields,
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
                                FieldAccess::<TypedNodeCommonFields>::new(
                                    Type::I32Type,
                                    Expression::FieldAccess(Rc::new(FieldAccess::<
                                        TypedNodeCommonFields,
                                    >::new(
                                        x_type.clone(),
                                        Expression::VariableReference(Rc::new(
                                            VariableReference::<TypedNodeCommonFields>::new(
                                                y_type.clone(),
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
        let sf_node = parse_and_check(test_inputs::OBJECT_INITIALIZATION_AND_FIELD_ACCESS);

        let x_type = Type::RecordType(Rc::new(RecordType::new(HashMap::from([(
            "c".to_string(),
            Type::I32Type,
        )]))));
        let y_type = Type::RecordType(Rc::new(RecordType::new(HashMap::from([
            ("a".to_string(), Type::BoolType),
            ("b".to_string(), x_type.clone()),
        ]))));

        assert_eq!(
            sf_node,
            Node::SourceFile(Rc::new(SourceFile::new(
                vec![Declaration::FunctionDeclaration(Rc::new(
                    FunctionDeclaration::<TypedNodeCommonFields>::new(
                        Type::FunctionType(Rc::new(FunctionType::new(vec![], Type::I32Type))),
                        Identifier::new("test".to_owned()),
                        Type::I32Type,
                        vec![],
                        Rc::new(Block::new_with_final_expression(
                            vec![Statement::Declaration(Declaration::VariableDeclaration(
                                Rc::new(VariableDeclaration::<TypedNodeCommonFields>::new(
                                    y_type.clone(),
                                    Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                                        y_type.clone(),
                                        Identifier::new("y".to_owned()),
                                        Type::TypeReference(Rc::new(TypeReference::new(
                                            Identifier::new("Y".to_owned()),
                                            vec![]
                                        )))
                                    )),
                                    Expression::ObjectLiteral(Rc::new(ObjectLiteral::<
                                        TypedNodeCommonFields,
                                    >::new(
                                        y_type.clone(),
                                        TypeReference::new(Identifier::new("Y".to_owned()), vec![]),
                                        HashMap::from([
                                            (
                                                "a".to_string(),
                                                Expression::BooleanLiteral(Rc::new(
                                                    BooleanLiteral::<TypedNodeCommonFields>::new(
                                                        true
                                                    )
                                                ))
                                            ),
                                            (
                                                "b".to_string(),
                                                Expression::ObjectLiteral(Rc::new(
                                                    ObjectLiteral::<TypedNodeCommonFields>::new(
                                                        x_type.clone(),
                                                        TypeReference::new(
                                                            Identifier::new("X".to_owned()),
                                                            vec![]
                                                        ),
                                                        HashMap::from([(
                                                            "c".to_string(),
                                                            Expression::IntegerLiteral(Rc::new(
                                                                IntegerLiteral::<
                                                                    TypedNodeCommonFields,
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
                                FieldAccess::<TypedNodeCommonFields>::new(
                                    Type::I32Type,
                                    Expression::FieldAccess(Rc::new(FieldAccess::<
                                        TypedNodeCommonFields,
                                    >::new(
                                        x_type.clone(),
                                        Expression::VariableReference(Rc::new(
                                            VariableReference::<TypedNodeCommonFields>::new(
                                                y_type.clone(),
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
                ))],
                vec![
                    ClassDeclaration::new(
                        Identifier::new("X".to_owned()),
                        vec![],
                        HashMap::from([("c".to_string(), Type::I32Type)]),
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
                                )))
                            )
                        ]),
                    ),
                ]
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
            Node::SourceFile(Rc::new(SourceFile::new(
                vec![Declaration::FunctionDeclaration(Rc::new(
                    FunctionDeclaration::<TypedNodeCommonFields>::new(
                        Type::FunctionType(Rc::new(FunctionType::new(
                            vec![Type::I32Type, Type::I32Type,],
                            Type::I32Type
                        ))),
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
                ))],
                vec![]
            )))
        );
    }

    #[test]
    fn function_call_add() {
        let sf_node = parse_and_check(test_inputs::FUNCTION_CALL_ADD);
        assert_eq!(
            sf_node,
            Node::SourceFile(Rc::new(SourceFile::new(
                vec![
                    Declaration::FunctionDeclaration(Rc::new(FunctionDeclaration::<
                        TypedNodeCommonFields,
                    >::new(
                        Type::FunctionType(Rc::new(FunctionType::new(
                            vec![Type::I32Type, Type::I32Type],
                            Type::I32Type
                        ))),
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
                    ))),
                    Declaration::FunctionDeclaration(Rc::new(FunctionDeclaration::<
                        TypedNodeCommonFields,
                    >::new(
                        Type::FunctionType(Rc::new(FunctionType::new(vec![], Type::I32Type))),
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
                    )),),
                ],
                vec![]
            )))
        );
    }

    #[test]
    fn function_call_update_state() {
        let sf_node = parse_and_check(test_inputs::FUNCTION_CALL_UPDATE_STATE);

        let c_type = Type::RecordType(Rc::new(RecordType::new(HashMap::from([(
            "a".to_string(),
            Type::I32Type,
        )]))));
        let c_dot_a_field_access =
            Expression::FieldAccess(Rc::new(FieldAccess::<TypedNodeCommonFields>::new(
                Type::I32Type,
                Expression::VariableReference(Rc::new(
                    VariableReference::<TypedNodeCommonFields>::new(
                        c_type.clone(),
                        Identifier::new("c".to_owned()),
                    ),
                )),
                "a".to_string(),
            )));

        assert_eq!(
            sf_node,
            Node::SourceFile(Rc::new(SourceFile::new(
                vec![
                    Declaration::FunctionDeclaration(Rc::new(FunctionDeclaration::<
                        TypedNodeCommonFields,
                    >::new(
                        Type::FunctionType(Rc::new(FunctionType::new(
                            vec![c_type.clone()],
                            c_type.clone(),
                        ))),
                        Identifier::new("updateState".to_owned()),
                        Type::TypeReference(Rc::new(TypeReference::new(
                            Identifier::new("C".to_owned()),
                            vec![Type::I32Type]
                        ))),
                        vec![Rc::new(Parameter::<TypedNodeCommonFields>::new(
                            c_type.clone(),
                            Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                                c_type.clone(),
                                Identifier::new("c".to_owned()),
                                Type::TypeReference(Rc::new(TypeReference::new(
                                    Identifier::new("C".to_owned()),
                                    vec![Type::I32Type]
                                )))
                            ))
                        ))],
                        Rc::new(Block::new_with_final_expression(
                            vec![],
                            Expression::ObjectLiteral(Rc::new(ObjectLiteral::<
                                TypedNodeCommonFields,
                            >::new(
                                c_type.clone(),
                                TypeReference::new(
                                    Identifier::new("C".to_owned()),
                                    vec![Type::I32Type]
                                ),
                                HashMap::from([(
                                    "a".to_string(),
                                    Expression::BinaryExpression(Rc::new(BinaryExpression::<
                                        TypedNodeCommonFields,
                                    >::new(
                                        Type::I32Type,
                                        c_dot_a_field_access,
                                        InfixOperator::Plus,
                                        Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                            TypedNodeCommonFields,
                                        >::new(
                                            1
                                        )))
                                    )))
                                ),])
                            )))
                        ))
                    ))),
                    Declaration::FunctionDeclaration(Rc::new(FunctionDeclaration::<
                        TypedNodeCommonFields,
                    >::new(
                        Type::FunctionType(Rc::new(FunctionType::new(vec![], Type::I32Type))),
                        Identifier::new("test".to_owned(),),
                        Type::I32Type,
                        vec![],
                        Rc::new(Block::new_with_final_expression(
                            vec![Statement::Declaration(Declaration::VariableDeclaration(
                                Rc::new(VariableDeclaration::<TypedNodeCommonFields>::new(
                                    c_type.clone(),
                                    Rc::new(VariableDeclarator::<TypedNodeCommonFields>::new(
                                        c_type.clone(),
                                        Identifier::new("c".to_owned()),
                                        Type::TypeReference(Rc::new(TypeReference::new(
                                            Identifier::new("C".to_owned()),
                                            vec![Type::I32Type]
                                        )))
                                    )),
                                    Expression::ObjectLiteral(Rc::new(ObjectLiteral::<
                                        TypedNodeCommonFields,
                                    >::new(
                                        c_type.clone(),
                                        TypeReference::new(
                                            Identifier::new("C".to_owned()),
                                            vec![Type::I32Type]
                                        ),
                                        HashMap::from([(
                                            "a".to_string(),
                                            Expression::IntegerLiteral(Rc::new(IntegerLiteral::<
                                                TypedNodeCommonFields,
                                            >::new(
                                                0
                                            )))
                                        ),])
                                    )))
                                ))
                            )),],
                            Expression::FieldAccess(Rc::new(
                                FieldAccess::<TypedNodeCommonFields>::new(
                                    Type::I32Type,
                                    Expression::FunctionCall(Rc::new(FunctionCall::<
                                        TypedNodeCommonFields,
                                    >::new(
                                        c_type.clone(),
                                        Identifier::new("updateState".to_owned()),
                                        vec![Expression::VariableReference(Rc::new(
                                            VariableReference::<TypedNodeCommonFields>::new(
                                                c_type.clone(),
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
