use std::collections::HashMap;

use crate::{Expression, Type, TypedNodeCommonFields, UntypedNodeCommonFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordLiteral<CommonFields: Clone> {
    pub common_fields: CommonFields,
    pub fields: HashMap<String, Expression<CommonFields>>,
}

impl RecordLiteral<UntypedNodeCommonFields> {
    pub fn new(fields: HashMap<String, Expression<UntypedNodeCommonFields>>) -> Self {
        RecordLiteral::<UntypedNodeCommonFields> {
            common_fields: UntypedNodeCommonFields::new(),
            fields,
        }
    }
}

impl RecordLiteral<TypedNodeCommonFields> {
    pub fn new(r#type: Type, fields: HashMap<String, Expression<TypedNodeCommonFields>>) -> Self {
        RecordLiteral::<TypedNodeCommonFields> {
            common_fields: TypedNodeCommonFields::new(r#type),
            fields,
        }
    }
}
