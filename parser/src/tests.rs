use crate::*;

mod tests {
    use super::*;

    #[test]
    fn create_parser() {
        let mut parser = Parser::new("");
        let output: Node = parser.parse();
        assert_eq!(output, SourceFile::new(Vec::new()));
    }

    fn define_main_body(body: Node) -> Node {
        SourceFile::new(vec![FunctionDeclarationStatement::new(
            Node::Identifier(Identifier {
                name: "main".to_owned(),
            }),
            Node::TypeLiteral(TypeLiteral::Int),
            vec![],
            body,
        )])
    }

    #[test]
    fn arithmetic_expressions() {
        let mut parser = Parser::new("func main(): int { 1; return 0; }");
        let mut output: Node = parser.parse();
        assert_eq!(
            output,
            define_main_body(Block::new(vec![
                Node::IntegerLiteral { value: 1 },
                ReturnStatement::new(Node::IntegerLiteral { value: 0 })
            ]))
        );

        parser = Parser::new("func main(): int { 2 + 2 * 2; return 0; }");
        output = parser.parse();
        assert_eq!(
            output,
            define_main_body(Block::new(vec![
                Node::BinaryExpression(BinaryExpression {
                    left: Box::new(Node::IntegerLiteral { value: 2 }),
                    operator: Box::new(Node::InfixOperator(InfixOperator::Plus)),
                    right: Box::new(Node::BinaryExpression(BinaryExpression {
                        left: Box::new(Node::IntegerLiteral { value: 2 }),
                        operator: Box::new(Node::InfixOperator(InfixOperator::Times)),
                        right: Box::new(Node::IntegerLiteral { value: 2 }),
                    })),
                }),
                ReturnStatement::new(Node::IntegerLiteral { value: 0 })
            ]))
        );

        parser = Parser::new("func main(): int { 2 % 2 + 2; return 0; }");
        output = parser.parse();
        assert_eq!(
            output,
            define_main_body(Block::new(vec![
                Node::BinaryExpression(BinaryExpression {
                    left: Box::new(Node::BinaryExpression(BinaryExpression {
                        left: Box::new(Node::IntegerLiteral { value: 2 }),
                        operator: Box::new(Node::InfixOperator(InfixOperator::Modulo)),
                        right: Box::new(Node::IntegerLiteral { value: 2 }),
                    })),
                    operator: Box::new(Node::InfixOperator(InfixOperator::Plus)),
                    right: Box::new(Node::IntegerLiteral { value: 2 }),
                }),
                ReturnStatement::new(Node::IntegerLiteral { value: 0 })
            ]))
        );
    }

    #[test]
    fn variable_declarations() {
        let mut parser = Parser::new("func main(): int { let a: int; return 0; }");
        let mut output: Node = parser.parse();
        assert_eq!(
            output,
            define_main_body(Block::new(vec![
                Node::VariableDeclarationStatement(VariableDeclarationStatement {
                    variable: Box::new(Node::VariableLike(VariableLike {
                        identifier: Box::new(Node::Identifier(Identifier {
                            name: "a".to_owned()
                        })),
                        variable_type: Box::new(Node::TypeLiteral(TypeLiteral::Int)),
                        initial_value: Box::new(None),
                    }))
                }),
                ReturnStatement::new(Node::IntegerLiteral { value: 0 })
            ]))
        );

        parser = Parser::new("func main(): int { let a: int = 5; return 0; }");
        output = parser.parse();
        assert_eq!(
            output,
            define_main_body(Block::new(vec![
                Node::VariableDeclarationStatement(VariableDeclarationStatement {
                    variable: Box::new(Node::VariableLike(VariableLike {
                        identifier: Box::new(Node::Identifier(Identifier {
                            name: "a".to_owned()
                        })),
                        variable_type: Box::new(Node::TypeLiteral(TypeLiteral::Int)),
                        initial_value: Box::new(Some(Node::IntegerLiteral { value: 5 })),
                    }))
                }),
                ReturnStatement::new(Node::IntegerLiteral { value: 0 })
            ]))
        );

        parser = Parser::new("func main(): int { let a: int = b; return 0; }");
        output = parser.parse();
        assert_eq!(
            output,
            define_main_body(Block::new(vec![
                Node::VariableDeclarationStatement(VariableDeclarationStatement {
                    variable: Box::new(Node::VariableLike(VariableLike {
                        identifier: Box::new(Node::Identifier(Identifier {
                            name: "a".to_owned()
                        })),
                        variable_type: Box::new(Node::TypeLiteral(TypeLiteral::Int)),
                        initial_value: Box::new(Some(VariableReference::new(Identifier::new(
                            "b".to_owned()
                        )))),
                    }))
                }),
                ReturnStatement::new(Node::IntegerLiteral { value: 0 })
            ]))
        );
    }

    #[test]
    fn function_declarations() {
        let mut parser = Parser::new("func a(b: int, c: int) : int { return b + c; }");
        let output = parser.parse();
        assert_eq!(
            output,
            SourceFile::new(vec![FunctionDeclarationStatement::new(
                Node::Identifier(Identifier {
                    name: "a".to_owned()
                }),
                Node::TypeLiteral(TypeLiteral::Int),
                vec![
                    Box::new(Node::VariableLike(VariableLike {
                        identifier: Box::new(Node::Identifier(Identifier {
                            name: "b".to_owned()
                        })),
                        variable_type: Box::new(Node::TypeLiteral(TypeLiteral::Int)),
                        initial_value: Box::new(None),
                    })),
                    Box::new(Node::VariableLike(VariableLike {
                        identifier: Box::new(Node::Identifier(Identifier {
                            name: "c".to_owned()
                        })),
                        variable_type: Box::new(Node::TypeLiteral(TypeLiteral::Int)),
                        initial_value: Box::new(None),
                    }))
                ],
                Block::new(vec![ReturnStatement::new(Node::BinaryExpression(
                    BinaryExpression {
                        left: Box::new(VariableReference::new(Identifier::new("b".to_owned()))),
                        operator: Box::new(Node::InfixOperator(InfixOperator::Plus)),
                        right: Box::new(VariableReference::new(Identifier::new("c".to_owned()))),
                    }
                ))])
            ),])
        );
    }

    #[test]
    fn function_calls() {
        let mut parser = Parser::new("func main(): int { a(b + 2); return 0; }");
        let output: Node = parser.parse();
        assert_eq!(
            output,
            define_main_body(Block::new(vec![
                FunctionCall::new(
                    Identifier::new("a".to_owned()),
                    vec![Node::BinaryExpression(BinaryExpression {
                        left: Box::new(VariableReference::new(Identifier::new("b".to_owned()))),
                        operator: Box::new(Node::InfixOperator(InfixOperator::Plus)),
                        right: Box::new(Node::IntegerLiteral { value: 2 }),
                    })],
                ),
                ReturnStatement::new(Node::IntegerLiteral { value: 0 })
            ]))
        );
    }

    #[test]
    fn if_statements() {
        let mut parser = Parser::new(
            "func dp(n: int): int {
                if (n == 0) {
                    return -1;
                }
                return n;
            }",
        );
        let output: Node = parser.parse();
        assert_eq!(
            output,
            SourceFile::new(vec![FunctionDeclarationStatement::new(
                Node::Identifier(Identifier {
                    name: "dp".to_owned(),
                }),
                Node::TypeLiteral(TypeLiteral::Int),
                vec![Box::new(Node::VariableLike(VariableLike {
                    identifier: Box::new(Node::Identifier(Identifier {
                        name: "n".to_owned()
                    })),
                    variable_type: Box::new(Node::TypeLiteral(TypeLiteral::Int)),
                    initial_value: Box::new(None),
                }))],
                Block::new(vec![
                    IfStatement::new(
                        BinaryExpression::new(
                            VariableReference::new(Identifier::new("n".to_owned())),
                            InfixOperator::new(InfixOperator::Equals),
                            Node::IntegerLiteral { value: 0 }
                        ),
                        Block::new(vec![ReturnStatement::new(PrefixExpression::new(
                            PrefixOperator::new(PrefixOperator::Minus),
                            Node::IntegerLiteral { value: 1 }
                        ))]),
                        None
                    ),
                    ReturnStatement::new(VariableReference::new(Identifier::new("n".to_owned())),)
                ])
            )])
        );
    }
}
