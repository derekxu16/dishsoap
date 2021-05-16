pub mod ast;
use ast::*;
use dishsoap_lexer::{Lexer, Token};

pub struct Parser<'ast> {
    lexer: Lexer<'ast>,
    scope_depth: i32,
}

impl<'ast> Parser<'ast> {
    pub fn new(source: &'ast str) -> Self {
        let parser = Parser {
            lexer: Lexer::new(source),
            scope_depth: 0,
        };

        parser
    }

    /// Used to keep track of the number of opening braces encountered.
    pub fn increase_scope_depth(&mut self) {
        self.scope_depth += 1;
    }

    /// Used to keep track of the number of closing braces encountered.
    pub fn decrease_scope_depth(&mut self) {
        self.scope_depth -= 1;
    }

    /// Get the current scope depth.
    pub fn get_scope_depth(&self) -> i32 {
        self.scope_depth
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

    /**
     * Tries to parse a VariableReference or FunctionCall, returns None if unsuccessful.
     */
    fn parse_reference(&mut self) -> Option<Node> {
        let identifier = self.parse_identifier();
        match identifier {
            Some(i) => match self.lexer.peek() {
                Some(Token::ParenOpen) => {
                    let arguments = FunctionCall::parse_arguments(self);
                    Some(FunctionCall::new(i, arguments))
                }
                _ => Some(VariableReference::new(i)),
            },
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

    fn parse_type(&mut self) -> Option<Node> {
        match self.lexer.peek() {
            Some(Token::IntKeyword) | Some(Token::VoidKeyword) => Some(TypeLiteral::parse(self)),
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
            Some(Token::Plus)
            | Some(Token::Minus)
            | Some(Token::Times)
            | Some(Token::Divide)
            | Some(Token::Percent)
            | Some(Token::DoubleEquals) => Some(InfixOperator::parse(self)),
            _ => None,
        }
    }

    /// Helper function to get operator precedence for Pratt parsing.
    fn get_precedence(&mut self) -> i32 {
        match self.lexer.peek() {
            Some(Token::Equals) => 1,
            Some(Token::DoubleEquals) => 2,
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
            .or_else(|| self.parse_reference())
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

    fn parse_variable_like(&mut self) -> Option<Node> {
        let variable = match self.lexer.peek() {
            Some(Token::Identifier) => Some(VariableLike::parse(self)),
            _ => None,
        };
        variable
    }

    fn parse_function_declaraction(&mut self) -> Option<Node> {
        match self.lexer.peek() {
            Some(Token::FuncKeyword) => Some(FunctionDeclarationStatement::parse(self)),
            _ => None,
        }
    }

    fn parse_statement(&mut self) -> Option<Node> {
        match self.lexer.peek() {
            Some(Token::LetKeyword) => Some(VariableDeclarationStatement::parse(self)),
            Some(Token::IfKeyword) => Some(IfStatement::parse(self)),
            Some(Token::ReturnKeyword) => Some(ReturnStatement::parse(self)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests;
