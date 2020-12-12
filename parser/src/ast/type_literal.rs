use super::super::{Parser, Token};
use super::node::{Node, Parsable};

#[derive(Debug, PartialEq, Eq)]
pub enum TypeLiteral {
    Int,
    Void,
}

impl Parsable for TypeLiteral {
    fn parse(parser: &mut Parser) -> Node {
        match parser.lexer.next() {
            Some(Token::IntKeyword) => Node::TypeLiteral(TypeLiteral::Int),
            Some(Token::VoidKeyword) => Node::TypeLiteral(TypeLiteral::Void),
            _ => panic!("Compilation error"),
        }
    }
}
