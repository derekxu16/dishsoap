use crate::{Identifier, Type, TypedNodeCommonFields, UntypedNodeCommonFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDeclarator<CommonFields> {
    pub common_fields: CommonFields,
    pub identifier: Identifier,
    pub variable_type: Type,
}

impl VariableDeclarator<UntypedNodeCommonFields> {
    pub fn new(identifier: Identifier, variable_type: Type) -> Self {
        VariableDeclarator::<UntypedNodeCommonFields> {
            common_fields: UntypedNodeCommonFields::new(),
            identifier,
            variable_type,
        }
    }
}

impl VariableDeclarator<TypedNodeCommonFields> {
    pub fn new(r#type: Type, identifier: Identifier, variable_type: Type) -> Self {
        VariableDeclarator::<TypedNodeCommonFields> {
            common_fields: TypedNodeCommonFields::new(r#type),
            identifier,
            variable_type,
        }
    }
}
