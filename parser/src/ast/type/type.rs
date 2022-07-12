use std::rc::Rc;

use crate::{FunctionType, RecordType, TypeLiteral};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    TypeLiteral(TypeLiteral),
    RecordType(Rc<RecordType>),
    FunctionType(Rc<FunctionType>),
}
