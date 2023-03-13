use std::collections::HashMap;

use crate::{
    ast::{Type, TypeReference},
    Expression, TypedNodeCommonFields, UntypedNodeCommonFields,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectLiteral<CommonFields: Clone> {
    pub common_fields: CommonFields,
    pub class: TypeReference,
    pub fields: HashMap<String, Expression<CommonFields>>,
}

impl ObjectLiteral<UntypedNodeCommonFields> {
    pub fn new(
        class: TypeReference,
        fields: HashMap<String, Expression<UntypedNodeCommonFields>>,
    ) -> Self {
        ObjectLiteral::<UntypedNodeCommonFields> {
            common_fields: UntypedNodeCommonFields::new(),
            class,
            fields,
        }
    }
}

impl ObjectLiteral<TypedNodeCommonFields> {
    pub fn new(
        r#type: Type,
        class: TypeReference,
        fields: HashMap<String, Expression<TypedNodeCommonFields>>,
    ) -> Self {
        ObjectLiteral::<TypedNodeCommonFields> {
            common_fields: TypedNodeCommonFields::new(r#type),
            class,
            fields,
        }
    }
}
