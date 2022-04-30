use crate::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionType {
    pub parameter_types: Vec<Type>,
    pub return_type: Type,
}

impl FunctionType {
    pub fn new(parameter_types: Vec<Type>, return_type: Type) -> Self {
        FunctionType {
            parameter_types,
            return_type,
        }
    }
}
