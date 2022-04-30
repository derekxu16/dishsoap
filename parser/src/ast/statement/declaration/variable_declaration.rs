use std::rc::Rc;

use crate::ast::{
    Expression, Type, TypedNodeCommonFields, UntypedNodeCommonFields, VariableDeclarator,
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
