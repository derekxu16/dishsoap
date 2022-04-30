use std::rc::Rc;

use crate::{Declaration, ReturnStatement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileStatement {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForStatement {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<CommonFields: Clone> {
    ReturnStatement(Rc<ReturnStatement<CommonFields>>),
    Declaration(Declaration<CommonFields>),
}
