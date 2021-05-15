use crate::ast::node::{Node, Parsable};
use crate::{Parser, Token};

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
pub struct VariableReference {
    pub identifier: Box<Node>,
}

impl VariableReference {
    pub fn new(identifier: Node) -> Node {
        Node::VariableReference(VariableReference {
            identifier: Box::new(identifier),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionCall {
    pub identifier: Box<Node>,
    pub arguments: Vec<Node>,
}

impl FunctionCall {
    pub fn new(identifier: Node, arguments: Vec<Node>) -> Node {
        Node::FunctionCall(FunctionCall {
            identifier: Box::new(identifier),
            arguments,
        })
    }

    pub fn parse_arguments(parser: &mut Parser) -> Vec<Node> {
        let mut arguments: Vec<Node> = Vec::new();
        parser.lexer.consume(Token::ParenOpen);
        loop {
            if parser.lexer.peek() == Some(Token::ParenClose) {
                parser.lexer.consume(Token::ParenClose);
                break;
            }
            let argument = parser.parse_expression(0);
            if argument.is_none() {
                panic!("Compilation error");
            }
            arguments.push(argument.unwrap());

            if parser.lexer.peek() == Some(Token::Comma) {
                parser.lexer.consume(Token::Comma);
            }
        }
        arguments
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
