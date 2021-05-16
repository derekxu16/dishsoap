use crate::ast::node::{Node, Parsable};
use crate::{Parser, Token};

#[derive(Debug, PartialEq, Eq)]
pub enum PrefixOperator {
    Plus,
    Minus,
    Bang,
}

impl PrefixOperator {
    pub fn new(operator: PrefixOperator) -> Node {
        Node::PrefixOperator(operator)
    }
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

impl InfixOperator {
    pub fn new(operator: InfixOperator) -> Node {
        Node::InfixOperator(operator)
    }
}

impl Parsable for InfixOperator {
    fn parse(parser: &mut Parser) -> Node {
        match parser.lexer.next() {
            Some(Token::Plus) => Node::InfixOperator(InfixOperator::Plus),
            Some(Token::Minus) => Node::InfixOperator(InfixOperator::Minus),
            Some(Token::Times) => Node::InfixOperator(InfixOperator::Times),
            Some(Token::Divide) => Node::InfixOperator(InfixOperator::Divide),
            Some(Token::Percent) => Node::InfixOperator(InfixOperator::Modulo),
            Some(Token::LessThan) => Node::InfixOperator(InfixOperator::LessThan),
            Some(Token::LessThanEquals) => Node::InfixOperator(InfixOperator::LessThanEquals),
            Some(Token::GreaterThan) => Node::InfixOperator(InfixOperator::GreaterThan),
            Some(Token::GreaterThanEquals) => Node::InfixOperator(InfixOperator::GreaterThanEquals),
            Some(Token::DoubleEquals) => Node::InfixOperator(InfixOperator::Equals),
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
    pub operator: Box<Node>,
    pub operand: Box<Node>,
}

impl PrefixExpression {
    pub fn new(operator: Node, operand: Node) -> Node {
        Node::PrefixExpression(PrefixExpression {
            operator: Box::new(operator),
            operand: Box::new(operand),
        })
    }
}

impl Parsable for PrefixExpression {
    fn parse(parser: &mut Parser) -> Node {
        let operator = parser.parse_prefix_operator();
        if operator.is_none() {
            panic!("Compilation error")
        }
        let operand = parser.parse_expression(0);
        if operand.is_none() {
            panic!("Compilation error")
        }

        PrefixExpression::new(operator.unwrap(), operand.unwrap())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BinaryExpression {
    pub left: Box<Node>,
    pub operator: Box<Node>,
    pub right: Box<Node>,
}

impl BinaryExpression {
    pub fn new(left: Node, operator: Node, right: Node) -> Node {
        Node::BinaryExpression(BinaryExpression {
            left: Box::new(left),
            operator: Box::new(operator),
            right: Box::new(right),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PostfixExpression {}
