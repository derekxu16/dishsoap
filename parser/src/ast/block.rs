use super::super::{Parser, Token};
use super::{Node, Parsable};

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    statements: Vec<Node>,
}

impl Block {
    pub fn new(statements: Vec<Node>) -> Node {
        Node::Block(Block { statements })
    }
}

impl Parsable for Block {
    fn parse(parser: &mut Parser) -> Node {
        let mut statements: Vec<Node> = Vec::new();

        let target_depth: i32 = parser.get_scope_depth();

        // The scope depth only needs to be increased when there are braces.
        // A source file contains a block too, but isn't usually surrounded by braces.
        if parser.lexer.peek() == Some(Token::BraceOpen) {
            parser.lexer.consume(Token::BraceOpen);
            parser.increase_scope_depth();
        }

        loop {
            match parser.lexer.peek() {
                Some(Token::BraceClose) => {
                    parser.lexer.consume(Token::BraceClose);
                    parser.decrease_scope_depth();
                    if parser.get_scope_depth() == target_depth {
                        break;
                    }
                }
                None => {
                    if parser.get_scope_depth() == 0 {
                        break;
                    } else {
                        panic!("Compilation error: unexpected end of file.");
                    }
                }
                _ => {}
            }
            let expression: Option<Node> = parser.parse_expression(0);
            if expression.is_some() {
                parser.lexer.consume(Token::Semicolon);
                statements.push(expression.unwrap());
                continue;
            }
            let statement: Option<Node> = parser.parse_statement();
            if statement.is_some() {
                statements.push(statement.unwrap());
                continue;
            }
            panic!("Compilation error")
        }

        Block::new(statements)
    }
}
