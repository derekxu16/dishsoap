use crate::{Expression, Type, TypedNodeCommonFields, UntypedNodeCommonFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldAccess<CommonFields: Clone> {
    pub common_fields: CommonFields,
    pub target: Expression<CommonFields>,
    pub field_name: String,
}

impl FieldAccess<UntypedNodeCommonFields> {
    pub fn new(target: Expression<UntypedNodeCommonFields>, field_name: String) -> Self {
        FieldAccess::<UntypedNodeCommonFields> {
            common_fields: UntypedNodeCommonFields::new(),
            target,
            field_name,
        }
    }
}

impl FieldAccess<TypedNodeCommonFields> {
    pub fn new(
        r#type: Type,
        target: Expression<TypedNodeCommonFields>,
        field_name: String,
    ) -> Self {
        FieldAccess::<TypedNodeCommonFields> {
            common_fields: TypedNodeCommonFields::new(r#type),
            target,
            field_name,
        }
    }
}
