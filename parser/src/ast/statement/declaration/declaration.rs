use std::rc::Rc;

use crate::{FunctionDeclaration, VariableDeclaration};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration<CommonFields: Clone> {
    FunctionDeclaration(Rc<FunctionDeclaration<CommonFields>>),
    VariableDeclaration(Rc<VariableDeclaration<CommonFields>>),
}
