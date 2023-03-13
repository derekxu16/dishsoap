use crate::ast::Identifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeReference {
    pub identifier: Identifier,
}

impl TypeReference {
    pub fn new(identifier: Identifier) -> Self {
        TypeReference { identifier }
    }
}
