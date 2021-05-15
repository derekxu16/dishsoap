use crate::ast::{Node, Parsable};
use crate::{Parser, Token};

#[derive(Debug, PartialEq, Eq)]
pub struct SourceFile {
    pub children: Vec<Node>,
}

impl SourceFile {
    pub fn new(children: Vec<Node>) -> Node {
        Node::SourceFile(SourceFile { children })
    }
}

impl Parsable for SourceFile {
    fn parse(parser: &mut Parser) -> Node {
        let mut children: Vec<Node> = Vec::new();

        loop {
            match parser.lexer.peek() {
                Some(Token::FuncKeyword) => (),
                None => {
                    break;
                }
                _ => panic!("Compilation error: unexpected token"),
            }
            let function_delcaration: Option<Node> = parser.parse_function_declaraction();
            if function_delcaration.is_none() {
                panic!("Compilation error")
            }
            children.push(function_delcaration.unwrap());
        }

        SourceFile::new(children)
    }
}
