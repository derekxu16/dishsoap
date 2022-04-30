use std::rc::Rc;

use crate::{
    Block, Expression, Identifier, Parameter, Type, TypedNodeCommonFields, UntypedNodeCommonFields,
    VariableDeclarator,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDeclaration<CommonFields: Clone> {
    pub common_fields: CommonFields,
    pub variable_declarator: Rc<VariableDeclarator<CommonFields>>,
    pub initial_value: Expression<CommonFields>,
}

impl VariableDeclaration<UntypedNodeCommonFields> {
    pub fn new(
        variable_declarator: Rc<VariableDeclarator<UntypedNodeCommonFields>>,
        initial_value: Expression<UntypedNodeCommonFields>,
    ) -> Self {
        VariableDeclaration::<UntypedNodeCommonFields> {
            common_fields: UntypedNodeCommonFields::new(),
            variable_declarator,
            initial_value,
        }
    }
}

impl VariableDeclaration<TypedNodeCommonFields> {
    pub fn new(
        r#type: Type,
        variable_declarator: Rc<VariableDeclarator<TypedNodeCommonFields>>,
        initial_value: Expression<TypedNodeCommonFields>,
    ) -> Self {
        VariableDeclaration::<TypedNodeCommonFields> {
            common_fields: TypedNodeCommonFields::new(r#type),
            variable_declarator,
            initial_value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDeclaration<CommonFields: Clone> {
    pub common_fields: CommonFields,
    pub identifier: Identifier,
    pub return_type: Type,
    pub parameters: Vec<Rc<Parameter<CommonFields>>>,
    pub body: Rc<Block<CommonFields>>,
}

impl FunctionDeclaration<UntypedNodeCommonFields> {
    pub fn new(
        identifier: Identifier,
        return_type: Type,
        parameters: Vec<Rc<Parameter<UntypedNodeCommonFields>>>,
        body: Rc<Block<UntypedNodeCommonFields>>,
    ) -> Self {
        FunctionDeclaration::<UntypedNodeCommonFields> {
            common_fields: UntypedNodeCommonFields::new(),
            identifier,
            return_type,
            parameters,
            body,
        }
    }
}

impl FunctionDeclaration<TypedNodeCommonFields> {
    pub fn new(
        r#type: Type,
        identifier: Identifier,
        return_type: Type,
        parameters: Vec<Rc<Parameter<TypedNodeCommonFields>>>,
        body: Rc<Block<TypedNodeCommonFields>>,
    ) -> Self {
        FunctionDeclaration::<TypedNodeCommonFields> {
            common_fields: TypedNodeCommonFields::new(r#type),
            identifier,
            return_type,
            parameters,
            body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration<CommonFields: Clone> {
    FunctionDeclaration(Rc<FunctionDeclaration<CommonFields>>),
    VariableDeclaration(Rc<VariableDeclaration<CommonFields>>),
}
