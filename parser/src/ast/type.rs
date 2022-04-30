use std::rc::Rc;

use crate::{FunctionType, TypeLiteral};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    FunctionType(Rc<FunctionType>),
    TypeLiteral(TypeLiteral),
}
