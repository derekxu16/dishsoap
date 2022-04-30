use crate::{Type, TypeLiteral, TypedNodeCommonFields, UntypedNodeCommonFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitLiteral<CommonFields> {
    pub common_fields: CommonFields,
}

impl UnitLiteral<UntypedNodeCommonFields> {
    pub fn new() -> Self {
        UnitLiteral::<UntypedNodeCommonFields> {
            common_fields: UntypedNodeCommonFields::new(),
        }
    }
}

impl UnitLiteral<TypedNodeCommonFields> {
    pub fn new() -> Self {
        UnitLiteral::<TypedNodeCommonFields> {
            common_fields: TypedNodeCommonFields::new(Type::TypeLiteral(TypeLiteral::UnitType)),
        }
    }
}
