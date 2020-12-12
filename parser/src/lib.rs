mod ast;
use ast::*;
use dishsoap_lexer::{Lexer, Token};

pub struct Parser<'ast> {
    lexer: Lexer<'ast>,
}

impl<'ast> Parser<'ast> {
    pub fn new(source: &'ast str) -> Self {
        let parser = Parser {
            lexer: Lexer::new(source),
        };

        parser
    }

    /// The entry point to begin parsing.
    pub fn parse(&mut self) -> Node {
        SourceFile::parse(self)
    }

    fn parse_identifier(&mut self) -> Option<Node> {
        match self.lexer.peek() {
            Some(Token::Identifier) => Some(Identifier::parse(self)),
            _ => None,
        }
    }

    fn parse_integer_literal(&mut self) -> Option<Node> {
        match self.lexer.peek() {
            Some(Token::IntegerLiteral) => {
                let value: String = self.lexer.slice().to_owned();
                self.lexer.consume(Token::IntegerLiteral);
                Some(Node::IntegerLiteral {
                    value: value.parse::<i32>().unwrap(),
                })
            }
            _ => None,
        }
    }

    fn parse_prefix_operator(&mut self) -> Option<Node> {
        let token: Option<Token> = self.lexer.peek();
        match token {
            Some(Token::Plus) | Some(Token::Minus) | Some(Token::Bang) => {
                Some(PrefixOperator::parse(self))
            }
            _ => None,
        }
    }

    pub fn parse_infix_operator(&mut self) -> Option<Node> {
        let token: Option<Token> = self.lexer.peek();
        match token {
            Some(Token::Plus) | Some(Token::Minus) | Some(Token::Times) | Some(Token::Divide)
            | Some(Token::Percent) => Some(InfixOperator::parse(self)),
            _ => None,
        }
    }

    /// Helper function to get operator precedence for Pratt parsing.
    fn get_precedence(&mut self) -> i32 {
        match self.lexer.peek() {
            Some(Token::Equals) => 1,
            Some(Token::Plus) | Some(Token::Minus) => 3,
            Some(Token::Times) | Some(Token::Divide) | Some(Token::Percent) => 4,
            _ => 0,
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Node> {
        match self.lexer.peek() {
            Some(Token::Plus) | Some(Token::Minus) | Some(Token::Bang) => {
                Some(PrefixExpression::parse(self))
            }
            _ => None,
        }
    }

    /// Parses an expression.
    // Uses Pratt parsing to handle operator precedence.
    // http://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/
    fn parse_expression(&mut self, precedence: i32) -> Option<Node> {
        let mut left = self
            .parse_prefix_expression()
            .or_else(|| self.parse_identifier())
            .or_else(|| self.parse_integer_literal())
            .or_else(|| match self.lexer.peek() {
                Some(Token::ParenOpen) => {
                    self.lexer.consume(Token::ParenOpen);
                    let expression: Option<Node> = self.parse_expression(precedence);
                    if expression.is_none() {
                        panic!("Compilation error");
                    }
                    self.lexer.consume(Token::ParenClose);

                    expression
                }
                _ => None,
            });

        loop {
            let next_precedence = self.get_precedence();
            if next_precedence <= precedence {
                // If the precedence of the next operator is too low, defer parsing.
                break;
            }

            let infix_operator = self.parse_infix_operator();

            left = Some(Node::BinaryExpression(BinaryExpression {
                left: Box::new(left.unwrap()),
                operator: Box::new(infix_operator.unwrap()),
                right: Box::new(self.parse_expression(next_precedence).unwrap()),
            }));
        }
        left
    }

    fn parse_statement(&mut self) -> Option<Node> {
        let token: Option<Token> = self.lexer.peek();
        match token {
            Some(Token::LetKeyword) => Some(VariableDeclarationStatement::parse(self)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests;
