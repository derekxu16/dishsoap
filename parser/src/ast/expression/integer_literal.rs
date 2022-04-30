use crate::{Type, TypeLiteral, TypedNodeCommonFields, UntypedNodeCommonFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteral<CommonFields> {
    pub common_fields: CommonFields,
    pub value: i32,
}

impl IntegerLiteral<UntypedNodeCommonFields> {
    pub fn new(value: i32) -> Self {
        IntegerLiteral::<UntypedNodeCommonFields> {
            common_fields: UntypedNodeCommonFields::new(),
            value,
        }
    }
}

impl IntegerLiteral<TypedNodeCommonFields> {
    pub fn new(value: i32) -> Self {
        IntegerLiteral::<TypedNodeCommonFields> {
            common_fields: TypedNodeCommonFields::new(Type::TypeLiteral(TypeLiteral::I32Type)),
            value,
        }
    }
}
