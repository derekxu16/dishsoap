use super::Type;
use crate::ast::Identifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeReference {
    pub identifier: Identifier,
    pub type_arguments: Vec<Type>,
}

impl TypeReference {
    pub fn new(identifier: Identifier, type_arguments: Vec<Type>) -> Self {
        TypeReference {
            identifier,
            type_arguments,
        }
    }
}
