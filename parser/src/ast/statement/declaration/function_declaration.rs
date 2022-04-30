use std::rc::Rc;

use crate::ast::{
    Block, Identifier, Parameter, Type, TypedNodeCommonFields, UntypedNodeCommonFields,
};

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
