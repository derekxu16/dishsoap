use crate::{Identifier, Type, TypedNodeCommonFields, UntypedNodeCommonFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableReference<CommonFields> {
    pub common_fields: CommonFields,
    pub identifier: Identifier,
}

impl VariableReference<UntypedNodeCommonFields> {
    pub fn new(identifier: Identifier) -> Self {
        VariableReference::<UntypedNodeCommonFields> {
            common_fields: UntypedNodeCommonFields::new(),
            identifier,
        }
    }
}

impl VariableReference<TypedNodeCommonFields> {
    pub fn new(r#type: Type, identifier: Identifier) -> Self {
        VariableReference::<TypedNodeCommonFields> {
            common_fields: TypedNodeCommonFields::new(r#type),
            identifier,
        }
    }
}
