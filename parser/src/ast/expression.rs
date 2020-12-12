use super::super::{Parser, Token};
use super::node::{Node, Parsable};

#[derive(Debug, PartialEq, Eq)]
pub enum PrefixOperator {
    Plus,
    Minus,
    Bang,
}

impl Parsable for PrefixOperator {
    fn parse(parser: &mut Parser) -> Node {
        match parser.lexer.next() {
            Some(Token::Plus) => Node::PrefixOperator(PrefixOperator::Plus),
            Some(Token::Minus) => Node::PrefixOperator(PrefixOperator::Minus),
            Some(Token::Bang) => Node::PrefixOperator(PrefixOperator::Bang),
            _ => panic!("Compilation error"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InfixOperator {
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    BitShiftLeft,
    BitShiftRight,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    Equals,
}

impl Parsable for InfixOperator {
    fn parse(parser: &mut Parser) -> Node {
        match parser.lexer.next() {
            Some(Token::Plus) => Node::InfixOperator(InfixOperator::Plus),
            Some(Token::Minus) => Node::InfixOperator(InfixOperator::Minus),
            Some(Token::Times) => Node::InfixOperator(InfixOperator::Times),
            Some(Token::Divide) => Node::InfixOperator(InfixOperator::Divide),
            Some(Token::Percent) => Node::InfixOperator(InfixOperator::Modulo),
            _ => panic!("Compilation error"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PrefixExpression {
    pub operator: Box<Option<Node>>,
    pub operand: Box<Option<Node>>,
}

impl Parsable for PrefixExpression {
    fn parse(parser: &mut Parser) -> Node {
        Node::PrefixExpression(PrefixExpression {
            operator: Box::new(parser.parse_prefix_operator()),
            operand: Box::new(parser.parse_expression(0)),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BinaryExpression {
    pub left: Box<Node>,
    pub operator: Box<Node>,
    pub right: Box<Node>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PostfixExpression {}
