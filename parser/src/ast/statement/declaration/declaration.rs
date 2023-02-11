use std::rc::Rc;

use super::{ClassDeclaration, FunctionDeclaration, VariableDeclaration};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration<CommonFields: Clone> {
    ClassDeclaration(Rc<ClassDeclaration>),
    FunctionDeclaration(Rc<FunctionDeclaration<CommonFields>>),
    VariableDeclaration(Rc<VariableDeclaration<CommonFields>>),
}
