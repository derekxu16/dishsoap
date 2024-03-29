use std::rc::Rc;

use super::{FunctionDeclaration, VariableDeclaration};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration<CommonFields: Clone> {
    FunctionDeclaration(Rc<FunctionDeclaration<CommonFields>>),
    VariableDeclaration(Rc<VariableDeclaration<CommonFields>>),
}
