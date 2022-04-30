use crate::{Expression, Identifier, Type, TypedNodeCommonFields, UntypedNodeCommonFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall<CommonFields: Clone> {
    pub common_fields: CommonFields,
    pub identifier: Identifier,
    pub arguments: Vec<Expression<CommonFields>>,
}

impl FunctionCall<UntypedNodeCommonFields> {
    pub fn new(
        identifier: Identifier,
        arguments: Vec<Expression<UntypedNodeCommonFields>>,
    ) -> Self {
        FunctionCall {
            common_fields: UntypedNodeCommonFields::new(),
            identifier,
            arguments,
        }
    }
}

impl FunctionCall<TypedNodeCommonFields> {
    pub fn new(
        r#type: Type,
        identifier: Identifier,
        arguments: Vec<Expression<TypedNodeCommonFields>>,
    ) -> Self {
        FunctionCall {
            common_fields: TypedNodeCommonFields::new(r#type),
            identifier,
            arguments,
        }
    }
}
