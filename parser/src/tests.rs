use super::{
    BinaryExpression, Block, FunctionDeclarationStatement, Identifier, InfixOperator, Node, Parser,
    ReturnStatement, SourceFile, TypeLiteral, VariableDeclarationStatement, VariableLike,
};

mod tests {
    use super::*;

    #[test]
    fn create_parser() {
        let mut parser = Parser::new("");
        let output: Node = parser.parse();
        assert_eq!(output, SourceFile::new(Block::new(Vec::new())));
    }

    #[test]
    fn arithmetic_expressions() {
        let mut parser = Parser::new("1;");
        let mut output: Node = parser.parse();
        assert_eq!(
            output,
            SourceFile::new(Block::new(vec![Node::IntegerLiteral { value: 1 }]))
        );

        parser = Parser::new("2 + 2 * 2;");
        output = parser.parse();
        assert_eq!(
            output,
            SourceFile::new(Block::new(vec![Node::BinaryExpression(BinaryExpression {
                left: Box::new(Node::IntegerLiteral { value: 2 }),
                operator: Box::new(Node::InfixOperator(InfixOperator::Plus)),
                right: Box::new(Node::BinaryExpression(BinaryExpression {
                    left: Box::new(Node::IntegerLiteral { value: 2 }),
                    operator: Box::new(Node::InfixOperator(InfixOperator::Times)),
                    right: Box::new(Node::IntegerLiteral { value: 2 }),
                })),
            })]))
        );

        parser = Parser::new("2 % 2 + 2;");
        output = parser.parse();
        assert_eq!(
            output,
            SourceFile::new(Block::new(vec![Node::BinaryExpression(BinaryExpression {
                left: Box::new(Node::BinaryExpression(BinaryExpression {
                    left: Box::new(Node::IntegerLiteral { value: 2 }),
                    operator: Box::new(Node::InfixOperator(InfixOperator::Modulo)),
                    right: Box::new(Node::IntegerLiteral { value: 2 }),
                })),
                operator: Box::new(Node::InfixOperator(InfixOperator::Plus)),
                right: Box::new(Node::IntegerLiteral { value: 2 }),
            })]))
        );
    }

    #[test]
    fn variable_declarations() {
        let mut parser = Parser::new("let a: int;");
        let mut output: Node = parser.parse();
        assert_eq!(
            output,
            SourceFile::new(Block::new(vec![Node::VariableDeclarationStatement(
                VariableDeclarationStatement {
                    variable: Box::new(Node::VariableLike(VariableLike {
                        identifier: Box::new(Node::Identifier(Identifier {
                            name: "a".to_owned()
                        })),
                        variable_type: Box::new(Node::TypeLiteral(TypeLiteral::Int)),
                        initial_value: Box::new(None),
                    }))
                }
            )]))
        );

        parser = Parser::new("let a: int = 5;");
        output = parser.parse();
        assert_eq!(
            output,
            SourceFile::new(Block::new(vec![Node::VariableDeclarationStatement(
                VariableDeclarationStatement {
                    variable: Box::new(Node::VariableLike(VariableLike {
                        identifier: Box::new(Node::Identifier(Identifier {
                            name: "a".to_owned()
                        })),
                        variable_type: Box::new(Node::TypeLiteral(TypeLiteral::Int)),
                        initial_value: Box::new(Some(Node::IntegerLiteral { value: 5 })),
                    }))
                }
            )]))
        );
    }

    #[test]
    fn function_declarations() {
        let mut parser = Parser::new("func a() : int { return 3; }");
        let mut output: Node = parser.parse();
        assert_eq!(
            output,
            SourceFile::new(Block::new(vec![FunctionDeclarationStatement::new(
                Node::Identifier(Identifier {
                    name: "a".to_owned()
                }),
                Node::TypeLiteral(TypeLiteral::Int),
                vec![],
                Block::new(vec![ReturnStatement::new(Node::IntegerLiteral {
                    value: 3
                })]),
            )]))
        );

        parser = Parser::new("func a(b: int, c: int) : int { return b + c; }");
        output = parser.parse();
        assert_eq!(
            output,
            SourceFile::new(Block::new(vec![FunctionDeclarationStatement::new(
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
                        left: Box::new(Node::Identifier(Identifier {
                            name: "b".to_owned()
                        })),
                        operator: Box::new(Node::InfixOperator(InfixOperator::Plus)),
                        right: Box::new(Node::Identifier(Identifier {
                            name: "c".to_owned()
                        })),
                    }
                ))])
            ),]))
        );
    }
}
