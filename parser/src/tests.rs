use super::{
    BinaryExpression, FunctionDeclarationStatement, Identifier, InfixOperator, Node, Parser,
    SourceFile, TypeLiteral, VariableDeclarationStatement, VariableLike,
};

mod tests {
    use super::*;

    #[test]
    fn create_parser() {
        let mut parser = Parser::new("");
        let output: Node = parser.parse();
        assert_eq!(
            output,
            Node::SourceFile(SourceFile {
                children: Vec::new()
            })
        );
    }

    #[test]
    fn arithmetic_expressions() {
        let mut parser = Parser::new("1;");
        let mut output: Node = parser.parse();
        assert_eq!(
            output,
            Node::SourceFile(SourceFile {
                children: vec![Node::IntegerLiteral { value: 1 }]
            })
        );

        parser = Parser::new("2 + 2 * 2;");
        output = parser.parse();
        assert_eq!(
            output,
            Node::SourceFile(SourceFile {
                children: vec![Node::BinaryExpression(BinaryExpression {
                    left: Box::new(Node::IntegerLiteral { value: 2 }),
                    operator: Box::new(Node::InfixOperator(InfixOperator::Plus)),
                    right: Box::new(Node::BinaryExpression(BinaryExpression {
                        left: Box::new(Node::IntegerLiteral { value: 2 }),
                        operator: Box::new(Node::InfixOperator(InfixOperator::Times)),
                        right: Box::new(Node::IntegerLiteral { value: 2 }),
                    })),
                })]
            })
        );

        parser = Parser::new("2 % 2 + 2;");
        output = parser.parse();
        assert_eq!(
            output,
            Node::SourceFile(SourceFile {
                children: vec![Node::BinaryExpression(BinaryExpression {
                    left: Box::new(Node::BinaryExpression(BinaryExpression {
                        left: Box::new(Node::IntegerLiteral { value: 2 }),
                        operator: Box::new(Node::InfixOperator(InfixOperator::Modulo)),
                        right: Box::new(Node::IntegerLiteral { value: 2 }),
                    })),
                    operator: Box::new(Node::InfixOperator(InfixOperator::Plus)),
                    right: Box::new(Node::IntegerLiteral { value: 2 }),
                })]
            })
        );
    }

    #[test]
    fn variable_declarations() {
        let mut parser = Parser::new("let a: int;");
        let mut output: Node = parser.parse();
        assert_eq!(
            output,
            Node::SourceFile(SourceFile {
                children: vec![Node::VariableDeclarationStatement(
                    VariableDeclarationStatement {
                        variable: Box::new(Node::VariableLike(VariableLike {
                            identifier: Box::new(Node::Identifier(Identifier {
                                name: "a".to_owned()
                            })),
                            variable_type: Box::new(Node::TypeLiteral(TypeLiteral::Int)),
                            initial_value: Box::new(None),
                        }))
                    }
                )]
            })
        );

        parser = Parser::new("let a: int = 5;");
        output = parser.parse();
        assert_eq!(
            output,
            Node::SourceFile(SourceFile {
                children: vec![Node::VariableDeclarationStatement(
                    VariableDeclarationStatement {
                        variable: Box::new(Node::VariableLike(VariableLike {
                            identifier: Box::new(Node::Identifier(Identifier {
                                name: "a".to_owned()
                            })),
                            variable_type: Box::new(Node::TypeLiteral(TypeLiteral::Int)),
                            initial_value: Box::new(Some(Node::IntegerLiteral { value: 5 })),
                        }))
                    }
                )]
            })
        );
    }

    #[test]
    fn function_declarations() {
        let mut parser = Parser::new("func a() : int {}");
        let mut output: Node = parser.parse();
        assert_eq!(
            output,
            Node::SourceFile(SourceFile {
                children: vec![Node::FunctionDeclarationStatement(
                    FunctionDeclarationStatement {
                        identifier: Box::new(Node::Identifier(Identifier {
                            name: "a".to_owned()
                        })),
                        parameters: vec![],
                        return_type: Box::new(Node::TypeLiteral(TypeLiteral::Int)),
                    }
                )]
            })
        );
        parser = Parser::new("func a(b: int, c: int) : int {}");
        output = parser.parse();
        assert_eq!(
            output,
            Node::SourceFile(SourceFile {
                children: vec![Node::FunctionDeclarationStatement(
                    FunctionDeclarationStatement {
                        identifier: Box::new(Node::Identifier(Identifier {
                            name: "a".to_owned()
                        })),
                        parameters: vec![
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
                        return_type: Box::new(Node::TypeLiteral(TypeLiteral::Int)),
                    }
                )]
            })
        );
    }
}
