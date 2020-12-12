use super::super::Parser;
use super::node::{Node, Parsable};

#[derive(Debug, PartialEq, Eq)]
pub struct Identifier {
    pub name: String,
}

impl Parsable for Identifier {
    fn parse(parser: &mut Parser) -> Node {
        let identifier: Node = Node::Identifier(Identifier {
            name: parser.lexer.slice().to_owned(),
        });
        parser.lexer.next();

        identifier
    }
}
