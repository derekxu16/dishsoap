use super::super::{Parser};
use super::{Block, Node, Parsable};

#[derive(Debug, PartialEq, Eq)]
pub struct SourceFile {
    pub children: Box<Node>,
}

impl SourceFile {
    pub fn new(children: Node) -> Node {
        Node::SourceFile(SourceFile {
            children: Box::new(children),
        })
    }
}

impl Parsable for SourceFile {
    fn parse(parser: &mut Parser) -> Node {
        SourceFile::new(Block::parse(parser))
    }
}
