use super::super::Parser;
use super::node::{Node, Parsable};

#[derive(Debug, PartialEq, Eq)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: String) -> Node {
        Node::Identifier(Identifier { name })
    }
}

impl Parsable for Identifier {
    fn parse(parser: &mut Parser) -> Node {
        let identifier: Node = Identifier::new(parser.lexer.slice().to_owned());
        parser.lexer.next();

        identifier
    }
}
