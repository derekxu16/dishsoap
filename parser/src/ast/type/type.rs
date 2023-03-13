use std::rc::Rc;

use super::{FunctionType, RecordType, TypeReference};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    UnitType,
    BoolType,
    I32Type,
    RecordType(Rc<RecordType>),
    FunctionType(Rc<FunctionType>),
    TypeReference(Rc<TypeReference>),
}
