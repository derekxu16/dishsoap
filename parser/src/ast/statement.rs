use super::super::{Parser, Token};
use super::{Node, Parsable};

#[derive(Debug, PartialEq, Eq)]
pub struct IfStatement {}

impl Parsable for IfStatement {
    fn parse(parser: &mut Parser) -> Node {
        parser.lexer.consume(Token::IfKeyword);
        parser.lexer.consume(Token::ParenOpen);
        Node::IfStatement(IfStatement {})
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct WhileStatement {}

#[derive(Debug, PartialEq, Eq)]
pub struct ForStatement {}

#[derive(Debug, PartialEq, Eq)]
pub struct ReturnStatement {}

#[derive(Debug, PartialEq, Eq)]
pub struct VariableDeclarationStatement {
    pub identifier: Box<Node>,
    pub variable_type: String,
    pub initial_value: Box<Option<Node>>,
}

impl Parsable for VariableDeclarationStatement {
    fn parse(parser: &mut Parser) -> Node {
        assert_eq!(parser.lexer.next(), Some(Token::LetKeyword));
        let identifier: Option<Node> = parser.parse_identifier();

        if identifier.is_none() {
            panic!("Compilation error");
        }

        parser.lexer.consume(Token::Colon);

        let variable_type: Option<String> = match parser.lexer.next() {
            Some(Token::IntType) => Some("int".to_owned()),
            Some(Token::VoidType) => Some("void".to_owned()),
            _ => None,
        };

        let initial_value = match parser.lexer.next() {
            Some(Token::Semicolon) => None,
            Some(Token::Equals) => {
                let expression = parser.parse_expression(0);
                parser.lexer.consume(Token::Semicolon);
                expression
            }
            _ => panic!("Compilation error"),
        };

        Node::VariableDeclarationStatement(VariableDeclarationStatement {
            identifier: Box::new(identifier.unwrap()),
            variable_type: variable_type.unwrap(),
            initial_value: Box::new(initial_value),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionDeclarationStatement {}
