use crate::{Type, TypedNodeCommonFields, UntypedNodeCommonFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BooleanLiteral<CommonFields> {
    pub common_fields: CommonFields,
    pub value: bool,
}

impl BooleanLiteral<UntypedNodeCommonFields> {
    pub fn new(value: bool) -> Self {
        BooleanLiteral::<UntypedNodeCommonFields> {
            common_fields: UntypedNodeCommonFields::new(),
            value,
        }
    }
}

impl BooleanLiteral<TypedNodeCommonFields> {
    pub fn new(value: bool) -> Self {
        BooleanLiteral::<TypedNodeCommonFields> {
            common_fields: TypedNodeCommonFields::new(Type::BoolType),
            value,
        }
    }
}
