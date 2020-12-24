use super::super::Parser;
use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    // Block
    Block(Block),
    // SourceFile
    SourceFile(SourceFile),
    // Operators
    PrefixOperator(PrefixOperator),
    InfixOperator(InfixOperator),
    // Literals
    IntegerLiteral { value: i32 },
    // Types
    TypeLiteral(TypeLiteral),
    // Identifier
    Identifier(Identifier),
    // Variables and parameters
    VariableLike(VariableLike),
    // Expressions
    PrefixExpression(PrefixExpression),
    BinaryExpression(BinaryExpression),
    // Statements
    IfStatement(IfStatement),
    ReturnStatement(ReturnStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    FunctionDeclarationStatement(FunctionDeclarationStatement),
}

pub trait Parsable {
    fn parse(parser: &mut Parser) -> Node;
}

trait Or: Sized {
    fn or(self, other: Self) -> Self;
}

impl<'a> Or for &'a Option<Node> {
    fn or(self, other: &'a Option<Node>) -> &'a Option<Node> {
        if self.is_none() {
            other
        } else {
            self
        }
    }
}
