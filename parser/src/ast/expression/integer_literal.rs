use crate::{Type, TypedNodeCommonFields, UntypedNodeCommonFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteral<CommonFields> {
    pub common_fields: CommonFields,
    pub value: i64,
}

impl IntegerLiteral<UntypedNodeCommonFields> {
    pub fn new(value: i64) -> Self {
        IntegerLiteral::<UntypedNodeCommonFields> {
            common_fields: UntypedNodeCommonFields::new(),
            value,
        }
    }
}

impl IntegerLiteral<TypedNodeCommonFields> {
    pub fn new(value: i64) -> Self {
        IntegerLiteral::<TypedNodeCommonFields> {
            common_fields: TypedNodeCommonFields::new(Type::I64Type),
            value,
        }
    }
}
