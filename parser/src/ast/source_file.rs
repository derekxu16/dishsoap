use super::super::{Parser, Token};
use super::node::{Node, Parsable};

#[derive(Debug, PartialEq, Eq)]
pub struct SourceFile {
    pub children: Vec<Node>,
}

impl SourceFile {
    pub fn new() -> SourceFile {
        SourceFile {
            children: Vec::new(),
        }
    }
}

impl Parsable for SourceFile {
    fn parse(parser: &mut Parser) -> Node {
        let mut source_file: SourceFile = SourceFile::new();
        while parser.lexer.peek() != None {
            let expression: Option<Node> = parser.parse_expression(0);
            if expression.is_some() {
                parser.lexer.consume(Token::Semicolon);
                source_file.children.push(expression.unwrap());
                continue;
            }
            let statement: Option<Node> = parser.parse_statement();
            if statement.is_some() {
                source_file.children.push(statement.unwrap());
                continue;
            }
            panic!("Compilation error")
        }

        Node::SourceFile(source_file)
    }
}
