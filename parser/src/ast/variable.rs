use super::super::{Parser, Token};
use super::{Node, Parsable};

#[derive(Debug, PartialEq, Eq)]
pub struct VariableLike {
    pub identifier: Box<Node>,
    pub variable_type: Box<Node>,
    pub initial_value: Box<Option<Node>>,
}

impl Parsable for VariableLike {
    fn parse(parser: &mut Parser) -> Node {
        let identifier: Option<Node> = parser.parse_identifier();
        if identifier.is_none() {
            panic!("Compilation error");
        }

        parser.lexer.consume(Token::Colon);

        let variable_type: Option<Node> = parser.parse_type();
        if variable_type.is_none() {
            panic!("Compilation error: expected type.")
        }
        let initial_value = match parser.lexer.peek() {
            Some(Token::Semicolon) | Some(Token::Comma) | Some(Token::ParenClose) => None,
            Some(Token::Equals) => {
                parser.lexer.consume(Token::Equals);
                let expression = parser.parse_expression(0);
                expression
            }
            _ => panic!("Compilation error"),
        };

        Node::VariableLike(VariableLike {
            identifier: Box::new(identifier.unwrap()),
            variable_type: Box::new(variable_type.unwrap()),
            initial_value: Box::new(initial_value),
        })
    }
}
