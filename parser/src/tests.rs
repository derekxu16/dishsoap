use super::{
    BinaryExpression, Identifier, InfixOperator, Node, Parser, SourceFile,
    VariableDeclarationStatement,
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
                        identifier: Box::new(Node::Identifier(Identifier {
                            name: "a".to_owned()
                        })),
                        variable_type: "int".to_owned(),
                        initial_value: Box::new(None),
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
                        identifier: Box::new(Node::Identifier(Identifier {
                            name: "a".to_owned()
                        })),
                        variable_type: "int".to_owned(),
                        initial_value: Box::new(Some(Node::IntegerLiteral { value: 5 })),
                    }
                )]
            })
        );
    }
}
