use std::rc::Rc;

use crate::{
    Block, Expression, Identifier, Parameter, SourceFile, Statement, Type, VariableDeclarator,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntypedNodeCommonFields {}

impl UntypedNodeCommonFields {
    const FLYWEIGHT_FIELDS: UntypedNodeCommonFields = UntypedNodeCommonFields {};

    pub fn new() -> UntypedNodeCommonFields {
        UntypedNodeCommonFields::FLYWEIGHT_FIELDS
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedNodeCommonFields {
    pub r#type: Type,
}

impl TypedNodeCommonFields {
    pub fn new(r#type: Type) -> TypedNodeCommonFields {
        TypedNodeCommonFields { r#type }
    }
}

/// This is essentially just a utility sum type that helps with polymorphism.
#[derive(Debug, Clone, PartialEq, Eq)]

pub enum Node<CommonFields: Clone> {
    // Identifier
    Identifier(Identifier),
    // Types
    Type(Type),
    // Expressions
    Expression(Expression<CommonFields>),
    // Variables and parameters
    VariableDeclarator(Rc<VariableDeclarator<CommonFields>>),
    Parameter(Rc<Parameter<CommonFields>>),
    // Block
    Block(Rc<Block<CommonFields>>),
    // Statements
    Statement(Statement<CommonFields>),
    // SourceFile
    SourceFile(Rc<SourceFile<CommonFields>>),
}
