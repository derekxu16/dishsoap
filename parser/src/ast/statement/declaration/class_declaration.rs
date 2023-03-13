use std::collections::HashMap;

use crate::{ast::Identifier, Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassDeclaration {
    pub identifier: Identifier,
    pub type_parameters: Vec<Identifier>,
    pub fields: HashMap<String, Type>,
}

impl ClassDeclaration {
    pub fn new(
        identifier: Identifier,
        type_parameters: Vec<Identifier>,
        fields: HashMap<String, Type>,
    ) -> Self {
        ClassDeclaration {
            identifier,
            type_parameters,
            fields,
        }
    }
}
