use std::rc::Rc;

use super::{Declaration, Expression};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileStatement {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForStatement {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStatement<CommonFields: Clone> {
    pub expression: Expression<CommonFields>,
}

impl<CommonFields: Clone> ReturnStatement<CommonFields> {
    pub fn new(expression: Expression<CommonFields>) -> Self {
        ReturnStatement::<CommonFields> { expression }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<CommonFields: Clone> {
    ReturnStatement(Rc<ReturnStatement<CommonFields>>),
    Declaration(Declaration<CommonFields>),
}
