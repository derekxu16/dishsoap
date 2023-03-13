use std::collections::HashMap;

use crate::{
    ast::{Identifier, Type},
    Expression, TypedNodeCommonFields, UntypedNodeCommonFields,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectLiteral<CommonFields: Clone> {
    pub common_fields: CommonFields,
    pub class_name: Identifier,
    pub fields: HashMap<String, Expression<CommonFields>>,
}

impl ObjectLiteral<UntypedNodeCommonFields> {
    pub fn new(
        class_name: Identifier,
        fields: HashMap<String, Expression<UntypedNodeCommonFields>>,
    ) -> Self {
        ObjectLiteral::<UntypedNodeCommonFields> {
            common_fields: UntypedNodeCommonFields::new(),
            class_name,
            fields,
        }
    }
}

impl ObjectLiteral<TypedNodeCommonFields> {
    pub fn new(
        r#type: Type,
        class_name: Identifier,
        fields: HashMap<String, Expression<TypedNodeCommonFields>>,
    ) -> Self {
        ObjectLiteral::<TypedNodeCommonFields> {
            common_fields: TypedNodeCommonFields::new(r#type),
            class_name,
            fields,
        }
    }
}
