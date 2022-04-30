use std::rc::Rc;

use super::{Type, TypedNodeCommonFields, UntypedNodeCommonFields, VariableDeclarator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter<CommonFields: Clone> {
    pub common_fields: CommonFields,
    pub variable_declarator: Rc<VariableDeclarator<CommonFields>>,
}

impl Parameter<UntypedNodeCommonFields> {
    pub fn new(variable_declarator: Rc<VariableDeclarator<UntypedNodeCommonFields>>) -> Self {
        Parameter::<UntypedNodeCommonFields> {
            common_fields: UntypedNodeCommonFields::new(),
            variable_declarator,
        }
    }
}

impl Parameter<TypedNodeCommonFields> {
    pub fn new(
        r#type: Type,
        variable_declarator: Rc<VariableDeclarator<TypedNodeCommonFields>>,
    ) -> Self {
        Parameter::<TypedNodeCommonFields> {
            common_fields: TypedNodeCommonFields::new(r#type),
            variable_declarator,
        }
    }
}
