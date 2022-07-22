use std::rc::Rc;

use crate::{FunctionType, RecordType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    UnitType,
    BoolType,
    I32Type,
    RecordType(Rc<RecordType>),
    FunctionType(Rc<FunctionType>),
}
